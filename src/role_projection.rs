use std::{
    collections::BTreeSet,
    fs,
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::{Component, Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use crate::generated as source;
use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use protos::{Protosizable, ReaderBudget, Textualizable};
use sha2::{Digest, Sha256};

use crate::{
    generated::{
        GeneratedRoleOutputDocument, GeneratedRoleOutputs, GeneratedRoleProvenance,
        GeneratedRoleProvenanceDocument, RoleOutputProvenance, RoleProjectionConfiguration,
        RoleSourceProvenance, RoleTools, Surface,
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
        let selection = source::RoleSelection {
            string: projection.role_selection.string.clone(),
            surface: projection.role_selection.surface.clone(),
        };
        let composition = compose_role(&source_socket, selection)?;
        if composition.first_string != projection.role_selection.string
            || composition.surface != projection.role_selection.surface
        {
            return Err(Error::RoleMismatch);
        }
        let text = render(&composition);
        let output_sha = sha256(text.as_bytes());
        let provenance = RoleOutputProvenance {
            first_string: relative.to_string_lossy().into_owned(),
            second_string: composition.first_string.clone(),
            surface: projection.role_selection.surface,
            third_string: output_sha,
            fourth_string: composition.fifth_string.clone(),
            fifth_string: composition.sixth_string.clone(),
            role_source_provenance_vector: composition
                .curriculum_context_provenance_vector
                .iter()
                .map(|item| RoleSourceProvenance {
                    first_string: item.first_string.clone(),
                    second_string: item.second_string.clone(),
                    third_string: item.third_string.clone(),
                    fourth_string: item.fourth_string.clone(),
                    fifth_string: item.fifth_string.clone(),
                    sixth_string: item.sixth_string.clone(),
                    seventh_string: item.seventh_string.clone(),
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

fn compose_role(
    socket: &Path,
    selection: source::RoleSelection,
) -> Result<source::CurriculumRoleComposition, Error> {
    let query = source::CurriculumQuery::ComposeRole(selection);
    let request = rkyv::to_bytes::<rkyv::rancor::Error>(&query)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?;
    let mut stream = UnixStream::connect(socket)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?;
    write_frame(&mut stream, &request)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?;
    let response = read_frame(&mut stream)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?;
    match rkyv::from_bytes::<source::CurriculumOutcome, rkyv::rancor::Error>(&response)
        .map_err(|error| Error::RoleSource(socket.to_path_buf(), error.to_string()))?
    {
        source::CurriculumOutcome::RoleComposed(composition) => Ok(composition),
        source::CurriculumOutcome::Rejected(reason) => Err(Error::RoleRejected(reason)),
        _ => Err(Error::RoleMismatch),
    }
}

fn render(role: &source::CurriculumRoleComposition) -> String {
    let effort = match role.effort {
        crate::generated::Effort::Low => "low",
        crate::generated::Effort::Medium => "medium",
        crate::generated::Effort::High => "high",
        crate::generated::Effort::Xhigh => "xhigh",
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
                role.first_string, role.third_string, role.second_string, role.fourth_string
            )
        }
        Surface::CodexAgent => format!(
            "name = {:?}\ndescription = {:?}\nmodel = {:?}\nmodel_reasoning_effort = {effort:?}\ndeveloper_instructions = {:?}\n",
            role.first_string,
            role.third_string,
            role.second_string,
            role.fourth_string.trim_end()
        ),
        Surface::PiAgent => {
            let restriction = if matches!(role.permission, crate::generated::Permission::Restricted)
            {
                "disallowed_tools: 'edit, write'\n"
            } else {
                ""
            };
            format!(
                "---\nname: {}\ndescription: {:?}\nmodel: 'openai-codex/{}'\nthinking: {effort}\nprojectRoleIdentity: {}\nprojectRoleDispatchKind: leaf\n{restriction}---\n\n{}",
                role.first_string,
                role.third_string,
                role.second_string,
                role.first_string,
                role.fourth_string
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
        reader: ReaderBudget {
            remaining: 1_048_576,
        },
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
    use std::{os::unix::net::UnixListener, thread};
    use tempfile::TempDir;

    #[test]
    fn destinations_stay_beneath_the_workspace() {
        assert!(safe_relative(Path::new(".pi/agents/read.md")).is_ok());
        assert!(safe_relative(Path::new("../outside")).is_err());
        assert!(safe_relative(Path::new("/outside")).is_err());
    }

    fn source(socket: PathBuf, outcomes: Vec<source::CurriculumOutcome>) -> thread::JoinHandle<()> {
        let listener = UnixListener::bind(socket).expect("mock Curriculum socket");
        thread::spawn(move || {
            for outcome in outcomes {
                let (mut stream, _) = listener.accept().expect("projection connects");
                let request = read_frame(&mut stream).expect("query frame");
                let _: source::CurriculumQuery =
                    rkyv::from_bytes::<_, rkyv::rancor::Error>(&request)
                        .expect("typed ComposeRole query");
                let response =
                    rkyv::to_bytes::<rkyv::rancor::Error>(&outcome).expect("typed outcome");
                write_frame(&mut stream, &response).expect("outcome frame");
            }
        })
    }

    fn composition(name: &str, surface: Surface) -> source::CurriculumOutcome {
        source::CurriculumOutcome::RoleComposed(source::CurriculumRoleComposition {
            first_string: name.into(),
            surface,
            second_string: "model".into(),
            effort: crate::generated::Effort::Medium,
            permission: crate::generated::Permission::Restricted,
            third_string: "reads a bounded target".into(),
            fourth_string: "context body\n".into(),
            role_tools: RoleTools::Inherit,
            role_callers: crate::generated::RoleCallers::Any,
            curriculum_context_provenance_vector: vec![source::CurriculumContextProvenance {
                first_string: "role-module".into(),
                second_string: "psyche-skills".into(),
                third_string: "revision".into(),
                fourth_string: "skills/role-module.md".into(),
                fifth_string: "source".into(),
                sixth_string: "body".into(),
                seventh_string: "rendered".into(),
                curriculum_startup_placement: source::CurriculumStartupPlacement::System,
                integer: 0,
            }],
            fifth_string: "description".into(),
            sixth_string: "context".into(),
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
                role_selection: crate::generated::RoleSelection {
                    string: name.into(),
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
            vec![source::CurriculumOutcome::Rejected(
                "unknown role/surface".into(),
            )],
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
