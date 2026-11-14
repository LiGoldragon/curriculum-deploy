use std::{
    collections::BTreeSet,
    fs,
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::{Component, Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use protos::{Protosizable, ReaderBudget, Textualizable};
use sha2::{Digest, Sha256};
use signal_curriculum_nexus::{
    Effort, Input, Output as CurriculumOutput, Permission, RoleComposition, RoleSelection,
    RoleTools, Surface,
};

use crate::{
    generated::{
        GeneratedRoleOutputDocument, GeneratedRoleOutputs, GeneratedRoleProvenance,
        GeneratedRoleProvenanceDocument, RoleOutputProvenance, RoleProjectionConfiguration,
        RoleSourceProvenance,
    },
    runtime::Error,
};

const INVENTORY: &str = "skills/generated-role-outputs.datom";
const PROVENANCE: &str = "skills/generated-role-provenance.datom";
static TEMPORARY: AtomicU64 = AtomicU64::new(0);

struct Output {
    relative: PathBuf,
    text: String,
    provenance: RoleOutputProvenance,
}

pub(crate) fn project(configuration: RoleProjectionConfiguration) -> Result<usize, Error> {
    let source_socket = PathBuf::from(configuration.first_string);
    let workspace = PathBuf::from(configuration.second_string);
    let previous = read_inventory(&workspace)?;
    let mut seen = BTreeSet::new();
    let mut outputs = Vec::new();

    for projection in configuration.role_projection_vector {
        let relative = safe_relative(Path::new(&projection.string))?;
        if !seen.insert(relative.clone()) {
            return Err(Error::Roles(format!(
                "duplicate role destination {}",
                relative.display()
            )));
        }
        let composition = compose_role(&source_socket, &projection.role_selection)?;
        if composition.role_name != projection.role_selection.role_name
            || composition.surface != projection.role_selection.surface
        {
            return Err(Error::RoleMismatch);
        }
        let text = render(&composition);
        let output_sha = sha256(text.as_bytes());
        let provenance = RoleOutputProvenance {
            first_string: relative.to_string_lossy().into_owned(),
            second_string: composition.role_name.0.clone(),
            surface: projection.role_selection.surface,
            third_string: output_sha,
            fourth_string: composition.description_payload_sha256.0.clone(),
            fifth_string: composition.context_payload_sha256.0.clone(),
            role_source_provenance_vector: composition
                .context_provenance_vector
                .iter()
                .map(|item| RoleSourceProvenance {
                    first_string: item.module_identity.0.clone(),
                    second_string: item.repository_name.0.clone(),
                    third_string: item.source_revision.0.clone(),
                    fourth_string: item.source_relative_path.0.clone(),
                    fifth_string: item.source_sha256.0.clone(),
                    sixth_string: item.body_sha256.0.clone(),
                    seventh_string: item.rendered_sha256.0.clone(),
                })
                .collect(),
        };
        outputs.push(Output {
            relative,
            text,
            provenance,
        });
    }

    for output in &outputs {
        atomic_write(&workspace, &output.relative, &output.text)?;
    }
    let current: Vec<String> = outputs
        .iter()
        .map(|output| output.relative.to_string_lossy().into_owned())
        .collect();
    for output in &outputs {
        let path = workspace.join(&output.relative);
        let read = fs::read(&path).map_err(|error| Error::Read(path.clone(), error))?;
        if sha256(&read) != output.provenance.third_string || read != output.text.as_bytes() {
            return Err(Error::Different(path));
        }
    }
    let current_set: BTreeSet<_> = current.iter().cloned().collect();
    for owned in previous {
        if !current_set.contains(&owned) {
            let relative = safe_relative(Path::new(&owned))?;
            let path = workspace.join(relative);
            if path.is_file() {
                fs::remove_file(&path).map_err(|error| Error::Write(path, error))?;
            }
        }
    }

    let inventory = GeneratedRoleOutputDocument::GeneratedRoleOutputs(GeneratedRoleOutputs {
        string_vector: current.clone(),
    });
    atomic_write(&workspace, Path::new(INVENTORY), &datom_text(&inventory))?;
    let provenance =
        GeneratedRoleProvenanceDocument::GeneratedRoleProvenance(GeneratedRoleProvenance {
            role_output_provenance_vector: outputs
                .iter()
                .map(|output| output.provenance.clone())
                .collect(),
        });
    atomic_write(&workspace, Path::new(PROVENANCE), &datom_text(&provenance))?;
    read_inventory(&workspace).and_then(|read| {
        if read == current {
            Ok(())
        } else {
            Err(Error::Different(workspace.join(INVENTORY)))
        }
    })?;
    Ok(outputs.len())
}

fn compose_role(socket: &Path, selection: &RoleSelection) -> Result<RoleComposition, Error> {
    let input = Input::ComposeRole(selection.clone());
    let request = rkyv::to_bytes::<rkyv::rancor::Error>(&input)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?;
    let mut stream = UnixStream::connect(socket)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?;
    write_frame(&mut stream, &request)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?;
    let response = read_frame(&mut stream)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?;
    match rkyv::from_bytes::<CurriculumOutput, rkyv::rancor::Error>(&response)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?
    {
        CurriculumOutput::RoleComposed(composition) => Ok(composition),
        CurriculumOutput::Rejected(reason) => Err(Error::RoleRejected(reason)),
        _ => Err(Error::RoleMismatch),
    }
}

fn render(role: &RoleComposition) -> String {
    let effort = match role.effort {
        Effort::Low => "low",
        Effort::Medium => "medium",
        Effort::High => "high",
        Effort::Xhigh => "xhigh",
    };
    let tools = match &role.role_tools {
        RoleTools::Inherit => None,
        RoleTools::Explicit(tools) => Some(tools.join(", ")),
    };
    match role.surface {
        Surface::ClaudeAgent => {
            let tools = tools
                .map(|tools| format!("tools: [{tools}]\n"))
                .unwrap_or_default();
            format!(
                "---\nname: {}\ndescription: {:?}\nmodel: {:?}\neffort: {effort}\n{tools}---\n\n{}",
                role.role_name.0, role.description_text.0, role.model_id.0, role.context_text.0
            )
        }
        Surface::CodexAgent => format!(
            "name = {:?}\ndescription = {:?}\nmodel = {:?}\nmodel_reasoning_effort = {effort:?}\ndeveloper_instructions = {:?}\n",
            role.role_name.0,
            role.description_text.0,
            role.model_id.0,
            role.context_text.0.trim_end()
        ),
        Surface::PiAgent => {
            let restriction = if matches!(role.permission, Permission::Restricted) {
                "disallowed_tools: 'edit, write'\n"
            } else {
                ""
            };
            format!(
                "---\nname: {}\ndescription: {:?}\nmodel: 'openai-codex/{}'\nthinking: {effort}\nprojectRoleIdentity: {}\nprojectRoleDispatchKind: leaf\n{restriction}---\n\n{}",
                role.role_name.0,
                role.description_text.0,
                role.model_id.0,
                role.role_name.0,
                role.context_text.0
            )
        }
    }
}

fn read_inventory(workspace: &Path) -> Result<Vec<String>, Error> {
    let path = workspace.join(INVENTORY);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let source = fs::read_to_string(&path).map_err(|error| Error::Read(path.clone(), error))?;
    let document = Potential::<GeneratedRoleOutputDocument>::from(source)
        .actualize(&mut budget())
        .map_err(|error| Error::Datom(datom_text(&error)))?;
    let GeneratedRoleOutputDocument::GeneratedRoleOutputs(outputs) = document;
    Ok(outputs.string_vector)
}

fn atomic_write(workspace: &Path, relative: &Path, body: &str) -> Result<(), Error> {
    let relative = safe_relative(relative)?;
    let path = workspace.join(relative);
    let parent = path.parent().expect("output parent");
    fs::create_dir_all(parent).map_err(|error| Error::Write(parent.to_path_buf(), error))?;
    let serial = TEMPORARY.fetch_add(1, Ordering::Relaxed);
    let temporary = parent.join(format!(
        ".curriculum-deploy-{}-{serial}.tmp",
        std::process::id()
    ));
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)
        .map_err(|error| Error::Write(temporary.clone(), error))?;
    file.write_all(body.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|error| Error::Write(temporary.clone(), error))?;
    fs::rename(&temporary, &path).map_err(|error| Error::Write(path, error))
}

fn safe_relative(relative: &Path) -> Result<PathBuf, Error> {
    if relative.as_os_str().is_empty()
        || relative.is_absolute()
        || relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(Error::Argument);
    }
    Ok(relative.to_path_buf())
}

fn write_frame(stream: &mut UnixStream, frame: &[u8]) -> std::io::Result<()> {
    let length =
        u32::try_from(frame.len()).map_err(|_| std::io::Error::other("frame too large"))?;
    stream.write_all(&length.to_be_bytes())?;
    stream.write_all(frame)?;
    stream.flush()
}

fn read_frame(stream: &mut UnixStream) -> std::io::Result<Vec<u8>> {
    let mut length = [0; 4];
    stream.read_exact(&mut length)?;
    let mut frame = vec![0; u32::from_be_bytes(length) as usize];
    stream.read_exact(&mut frame)?;
    Ok(frame)
}

fn budget() -> Budget {
    Budget {
        remaining: 1_048_576,
        reader: ReaderBudget::try_from(1_048_576).expect("a constant reader budget"),
        depth: 0,
        maximum_depth: 1_024,
    }
}

fn datom_text<T: Datomizable>(value: &T) -> String {
    value.datomize(vec![]).protosize().textualize()
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::RoleProjection;
    use signal_curriculum_nexus::{
        BodySha256, ContextPayloadSha256, ContextProvenance, ContextText, DescriptionPayloadSha256,
        DescriptionText, ModelId, ModuleIdentity, RenderedSha256, RepositoryName, RoleCallers,
        RoleName, SourceRelativePath, SourceRevision, SourceSha256, StartupPlacement,
    };
    use std::{os::unix::net::UnixListener, thread};
    use tempfile::TempDir;

    #[test]
    fn destinations_stay_beneath_the_workspace() {
        assert!(safe_relative(Path::new(".pi/agents/read.md")).is_ok());
        assert!(safe_relative(Path::new("../outside")).is_err());
        assert!(safe_relative(Path::new("/outside")).is_err());
    }

    fn source(socket: PathBuf, outputs: Vec<CurriculumOutput>) -> thread::JoinHandle<()> {
        let listener = UnixListener::bind(socket).expect("mock Curriculum socket");
        thread::spawn(move || {
            for output in outputs {
                let (mut stream, _) = listener.accept().expect("projection connects");
                let request = read_frame(&mut stream).expect("input frame");
                let _: Input = rkyv::from_bytes::<_, rkyv::rancor::Error>(&request)
                    .expect("typed ComposeRole input");
                let response =
                    rkyv::to_bytes::<rkyv::rancor::Error>(&output).expect("typed output");
                write_frame(&mut stream, &response).expect("outcome frame");
            }
        })
    }

    fn composition(name: &str, surface: Surface) -> CurriculumOutput {
        CurriculumOutput::RoleComposed(RoleComposition {
            role_name: RoleName(name.into()),
            surface,
            model_id: ModelId("model".into()),
            effort: Effort::Medium,
            permission: Permission::Restricted,
            description_text: DescriptionText("reads a bounded target".into()),
            context_text: ContextText("context body\n".into()),
            role_tools: RoleTools::Inherit,
            role_callers: RoleCallers::Any,
            context_provenance_vector: vec![ContextProvenance {
                module_identity: ModuleIdentity("role-module".into()),
                repository_name: RepositoryName("psyche-skills".into()),
                source_revision: SourceRevision("revision".into()),
                source_relative_path: SourceRelativePath("skills/role-module.md".into()),
                source_sha256: SourceSha256("source".into()),
                body_sha256: BodySha256("body".into()),
                rendered_sha256: RenderedSha256("rendered".into()),
                startup_placement: StartupPlacement::System,
                integer: 0,
            }],
            description_payload_sha256: DescriptionPayloadSha256("description".into()),
            context_payload_sha256: ContextPayloadSha256("context".into()),
        })
    }

    fn configuration(
        socket: &Path,
        workspace: &Path,
        name: &str,
        destination: &str,
    ) -> RoleProjectionConfiguration {
        RoleProjectionConfiguration {
            first_string: socket.to_string_lossy().into_owned(),
            second_string: workspace.to_string_lossy().into_owned(),
            role_projection_vector: vec![RoleProjection {
                role_selection: RoleSelection {
                    role_name: RoleName(name.into()),
                    surface: Surface::PiAgent,
                },
                string: destination.into(),
            }],
        }
    }

    #[test]
    fn projection_reads_back_hashes_cleans_only_owned_outputs_and_preserves_unrelated_files() {
        let root = TempDir::new().expect("temporary roots");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(workspace.join(".pi/agents")).expect("workspace");
        let unrelated = workspace.join(".pi/agents/home-owned.md");
        fs::write(&unrelated, "home owned").expect("unrelated file");

        let first_socket = root.path().join("first.sock");
        let first = source(
            first_socket.clone(),
            vec![composition("read-trivial", Surface::PiAgent)],
        );
        let retired = ".pi/agents/read-trivial.md";
        assert_eq!(
            project(configuration(
                &first_socket,
                &workspace,
                "read-trivial",
                retired
            ))
            .expect("first projection"),
            1
        );
        first.join().expect("first source");

        let second_socket = root.path().join("second.sock");
        let second = source(
            second_socket.clone(),
            vec![composition("write-trivial", Surface::PiAgent)],
        );
        let current = ".pi/agents/write-trivial.md";
        project(configuration(
            &second_socket,
            &workspace,
            "write-trivial",
            current,
        ))
        .expect("replacement projection");
        second.join().expect("second source");

        assert!(
            !workspace.join(retired).exists(),
            "recorded owned output cleaned"
        );
        assert_eq!(fs::read_to_string(unrelated).unwrap(), "home owned");
        let body = fs::read_to_string(workspace.join(current)).expect("projected role");
        let provenance = fs::read_to_string(workspace.join(PROVENANCE)).expect("provenance");
        assert!(provenance.contains(&sha256(body.as_bytes())));
        assert!(provenance.contains("revision"));
        assert!(!provenance.contains(first_socket.to_string_lossy().as_ref()));
    }

    #[test]
    fn source_rejection_is_a_typed_failure_and_writes_nothing() {
        let root = TempDir::new().expect("temporary roots");
        let socket = root.path().join("curriculum.sock");
        let server = source(
            socket.clone(),
            vec![CurriculumOutput::Rejected("unknown role/surface".into())],
        );
        let workspace = root.path().join("workspace");
        let result = project(configuration(
            &socket,
            &workspace,
            "missing",
            ".pi/agents/missing.md",
        ));
        server.join().expect("source");
        assert!(
            matches!(result, Err(Error::RoleRejected(reason)) if reason == "unknown role/surface")
        );
        assert!(!workspace.exists());
    }
}
