mod support;

use std::{collections::BTreeSet, fs, path::Path, process::Command};

use curriculum::generated::RolesDocument;
use curriculum_deploy::generated::GeneratedRoleOutputDocument;
use datom_codec::{Actualizing, Budget, Potential};
use protos::ReaderBudget;
use support::StandInCurriculum;
use tempfile::tempdir;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_curriculum-deploy")
}

fn data_root() -> String {
    std::env::var("CURRICULUM_TEST_DATA_ROOT")
        .unwrap_or_else(|_| "/external-fixture-not-configured".into())
}

/// The three belief repositories of the external data, each declared under
/// its aspect.
fn belief_sources() -> Vec<(&'static str, std::path::PathBuf)> {
    [
        ("Psyche", "CURRICULUM_TEST_PSYCHE_BELIEF"),
        ("Mind", "CURRICULUM_TEST_MIND_BELIEF"),
        ("Field", "CURRICULUM_TEST_FIELD_BELIEF"),
    ]
    .into_iter()
    .map(|(aspect, variable)| {
        (
            aspect,
            std::env::var(variable)
                .unwrap_or_else(|_| "/external-fixture-not-configured".into())
                .into(),
        )
    })
    .collect()
}

/// The external data: Curriculum's checkout for its roles, and the belief
/// repositories for the modules; the role packets come from the Curriculum
/// Nexus at `socket`.
fn request(operation: &str, workspace: &Path, socket: &Path) -> String {
    let data_root = data_root();
    let sources = belief_sources();
    let sources: Vec<(&str, &Path)> = sources
        .iter()
        .map(|(aspect, path)| (*aspect, path.as_path()))
        .collect();
    deployment_request(
        operation,
        Path::new(&data_root),
        &sources,
        workspace,
        socket,
    )
}

fn deployment_request(
    operation: &str,
    curriculum: &Path,
    sources: &[(&str, &Path)],
    workspace: &Path,
    socket: &Path,
) -> String {
    let sources = sources
        .iter()
        .map(|(aspect, directory)| format!("{aspect}.«{}»", directory.display()))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "{operation}.{{ «{}» [ {sources} ] «{}» «{}» }}",
        curriculum.display(),
        workspace.display(),
        socket.display()
    )
}

/// Every module the belief repositories declare in their modules.datom, of
/// the kinds the runtime reads: `<kind>:<subject>:<version>`, one per
/// declared version, with the file holding its text.
fn declared_modules() -> std::collections::BTreeMap<String, std::path::PathBuf> {
    const KINDS: [&str; 6] = [
        "Compensation",
        "Intent",
        "Knowledge",
        "Role",
        "Spirit",
        "Vision",
    ];
    let mut modules = std::collections::BTreeMap::new();
    for (_, source) in belief_sources() {
        let declarations =
            fs::read_to_string(source.join("modules.datom")).expect("module declarations");
        for declaration in declarations.split('{').skip(1) {
            let words: Vec<&str> = declaration
                .split(|c: char| c.is_whitespace() || c == '[' || c == ']')
                .filter(|word| !word.is_empty())
                .collect();
            let [kind, subject, rest @ ..] = words.as_slice() else {
                continue;
            };
            if !KINDS.contains(kind) {
                continue;
            }
            for version in rest
                .iter()
                .take_while(|word| word.bytes().all(|b| b.is_ascii_digit()))
            {
                let kind = kind.to_lowercase();
                modules.insert(
                    format!("{kind}:{subject}:{version}"),
                    source
                        .join(&kind)
                        .join(subject)
                        .join(format!("{version}.md")),
                );
            }
        }
    }
    modules
}

fn generated_skill_names(workspace: &Path, surface: &str) -> BTreeSet<String> {
    fs::read_dir(workspace.join(surface))
        .expect("generated skill surface")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join("SKILL.md").is_file())
        .map(|entry| entry.file_name().into_string().expect("UTF-8 skill name"))
        .collect()
}

/// The path of a subject's version 0 under a module-kind directory, its
/// subject directory made.
fn version_zero(kind_directory: &Path, subject: &str) -> std::path::PathBuf {
    let directory = kind_directory.join(subject);
    fs::create_dir_all(&directory).expect("subject directory");
    directory.join("0.md")
}

#[test]
#[ignore = "requires the externally owned Curriculum data fixture"]
fn external_data_generates_skills_roles_and_a_typed_cleanup_inventory() {
    let workspace = tempdir().expect("workspace");
    let unrelated = tempdir().expect("unrelated current directory");
    let runtime = tempdir().expect("Curriculum runtime");
    let curriculum =
        StandInCurriculum::composing_every_role(&runtime.path().join("curriculum.sock"));
    let output = Command::new(binary())
        .arg(request("Generate", workspace.path(), &curriculum.socket))
        .current_dir(unrelated.path())
        .env_clear()
        .env("SKILLS_SOURCE_ROOT", "/poison")
        .env("SKILLS_WORKSPACE_ROOT", "/poison")
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");

    let modules = declared_modules();
    let declared: BTreeSet<String> = modules.keys().cloned().collect();
    assert!(
        !declared.is_empty(),
        "the belief repositories declare modules"
    );
    for surface in [".agents/skills", ".claude/skills", ".opencode/skills"] {
        assert_eq!(
            generated_skill_names(workspace.path(), surface),
            declared,
            "{surface} must cover exactly the modules the belief repositories declare"
        );
    }
    assert!(!workspace.path().join("flows").exists());
    for (name, source) in &modules {
        let user_only = fs::read_to_string(source)
            .expect("module source")
            .lines()
            .any(|line| line == "user-only: true");
        let skill = fs::read_to_string(
            workspace
                .path()
                .join(".claude/skills")
                .join(name)
                .join("SKILL.md"),
        )
        .expect("Claude skill");
        assert!(!skill.contains("\nuser-only: true\n"), "{name}");
        assert_eq!(
            workspace
                .path()
                .join(".agents/skills")
                .join(name)
                .join("agents/openai.yaml")
                .is_file(),
            user_only,
            "{name}: a user-only module, and only one, keeps an explicit Codex route"
        );
    }

    let generated_role_files: Vec<_> = [".claude/agents", ".codex/agents", ".pi/agents"]
        .into_iter()
        .flat_map(|relative| fs::read_dir(workspace.path().join(relative)).expect("role directory"))
        .collect();
    let roles_source = fs::read_to_string(std::path::Path::new(&data_root()).join("roles.datom"))
        .expect("authored fixture roles");
    let RolesDocument::Roles(authored_roles) = Potential::<RolesDocument>::from(roles_source)
        .actualize(&mut Budget {
            remaining: 16_384,
            reader: ReaderBudget::try_from(16_384).expect("a constant reader budget"),
            depth: 0,
            maximum_depth: 16_384,
        })
        .expect("authored fixture roles compose");
    assert_eq!(
        generated_role_files.len(),
        authored_roles.role_definition_vector.len(),
        "every declared role row produces one role file"
    );
    for role in &authored_roles.role_definition_vector {
        let relative = match role.surface {
            signal_curriculum_nexus::Surface::ClaudeAgent => {
                format!(".claude/agents/{}.md", role.role_name.0)
            }
            signal_curriculum_nexus::Surface::CodexAgent => {
                format!(".codex/agents/{}.toml", role.role_name.0)
            }
            signal_curriculum_nexus::Surface::PiAgent => {
                format!(".pi/agents/{}.md", role.role_name.0)
            }
        };
        assert!(workspace.path().join(relative).is_file());
    }
    let inventory =
        fs::read_to_string(workspace.path().join("skills/generated-role-outputs.datom"))
            .expect("typed cleanup inventory");
    let GeneratedRoleOutputDocument::GeneratedRoleOutputs(inventory) =
        Potential::<GeneratedRoleOutputDocument>::from(inventory)
            .actualize(&mut Budget {
                remaining: 16_384,
                reader: ReaderBudget::try_from(16_384).expect("a constant reader budget"),
                depth: 0,
                maximum_depth: 16_384,
            })
            .expect("generated role inventory composes");
    assert_eq!(
        inventory.string_vector.len(),
        authored_roles.role_definition_vector.len()
    );

    let retired_agent_skill = workspace.path().join(".agents/skills/flows/SKILL.md");
    let retired_claude_skill = workspace.path().join(".claude/skills/subflows/SKILL.md");
    fs::create_dir_all(retired_agent_skill.parent().expect("retired agent parent"))
        .expect("retired agent skill parent");
    fs::create_dir_all(
        retired_claude_skill
            .parent()
            .expect("retired Claude parent"),
    )
    .expect("retired Claude skill parent");
    fs::write(&retired_agent_skill, "retired").expect("retired agent skill");
    fs::write(&retired_claude_skill, "retired").expect("retired Claude skill");

    let stale = workspace.path().join(".codex/agents/retired.toml");
    fs::write(&stale, "retired").expect("stale role");
    fs::write(
        workspace.path().join("skills/generated-role-outputs.datom"),
        "GeneratedRoleOutputs.{ [ «.codex/agents/retired.toml» ] }",
    )
    .expect("prior typed inventory");
    let output = Command::new(binary())
        .arg(request("Generate", workspace.path(), &curriculum.socket))
        .output()
        .expect("runtime regenerates");
    assert!(output.status.success(), "{output:?}");
    assert!(!stale.exists());
    assert!(!retired_agent_skill.exists());
    assert!(!retired_claude_skill.exists());
    let output = Command::new(binary())
        .arg(request("Check", workspace.path(), &curriculum.socket))
        .output()
        .expect("runtime checks generated output");
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn cli_accepts_only_one_inline_object() {
    let workspace = tempdir().expect("workspace");
    let valid = request(
        "Visualize",
        workspace.path(),
        &workspace.path().join("curriculum.sock"),
    );
    for arguments in [
        Vec::new(),
        vec![valid.clone(), valid.clone()],
        vec!["--generate".into()],
        vec!["request.datom".into()],
    ] {
        let output = Command::new(binary())
            .args(arguments)
            .output()
            .expect("runtime starts");
        assert!(!output.status.success());
    }
}

#[test]
fn skill_conditionals_render_only_for_their_target() {
    let data = tempdir().expect("data root");
    let source = data.path().join("field");
    let skills = source.join("compensation");
    fs::create_dir_all(&skills).expect("skills directory");
    fs::write(data.path().join("roles.datom"), "Roles.{ [] }").expect("empty role data");
    fs::write(
        version_zero(&skills, "commands"),
        "Shared command.\n{% if claude %}\nclaude command\n{% endif %}\n{% if codex %}\ncodex command\n{% endif %}\n{% if opencode %}\nopencode command\n{% endif %}\n{% if pi %}\npi command\n{% endif %}\n",
    )
    .expect("conditional skill");

    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            data.path(),
            &[("Field", &source)],
            workspace.path(),
            &data.path().join("curriculum.sock"),
        ))
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");

    let claude = fs::read_to_string(
        workspace
            .path()
            .join(".claude/skills/compensation:commands:0/SKILL.md"),
    )
    .expect("Claude skill");
    assert!(claude.contains("claude command"));
    assert!(!claude.contains("codex command"));
    assert!(!claude.contains("opencode command"));
    assert!(!claude.contains("pi command"));

    let codex = fs::read_to_string(
        workspace
            .path()
            .join(".agents/skills/compensation:commands:0/SKILL.md"),
    )
    .expect("Codex skill");
    assert!(codex.contains("codex command"));
    assert!(!codex.contains("claude command"));
    assert!(!codex.contains("opencode command"));
    assert!(!codex.contains("pi command"));

    let opencode = fs::read_to_string(
        workspace
            .path()
            .join(".opencode/skills/compensation:commands:0/SKILL.md"),
    )
    .expect("OpenCode skill");
    assert_eq!(
        opencode, "---\nname: compensation:commands:0\n---\nShared command.\nopencode command\n",
        "OpenCode receives only its own block, under frontmatter that names the skill"
    );
}

#[test]
fn opencode_skill_frontmatter_names_the_skill_and_other_harnesses_are_unchanged() {
    let data = tempdir().expect("data root");
    let source = data.path().join("psyche");
    let skills = source.join("spirit");
    fs::create_dir_all(&skills).expect("skills directory");
    fs::write(data.path().join("roles.datom"), "Roles.{ [] }").expect("empty role data");
    let body = "---\ndescription: Every agent task.\ndependencies: [behavior]\n---\n\nThe body.\n";
    fs::write(version_zero(&skills, "spirit"), body).expect("spirit skill");

    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            data.path(),
            &[("Psyche", &source)],
            workspace.path(),
            &data.path().join("curriculum.sock"),
        ))
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");

    assert_eq!(
        fs::read_to_string(
            workspace
                .path()
                .join(".opencode/skills/spirit:spirit:0/SKILL.md"),
        )
        .expect("OpenCode skill"),
        "---\nname: spirit:spirit:0\ndescription: Every agent task.\ndependencies: [behavior]\n---\n\nThe body.\n"
    );
    for surface in [".agents/skills", ".claude/skills"] {
        assert_eq!(
            fs::read_to_string(
                workspace
                    .path()
                    .join(surface)
                    .join("spirit:spirit:0/SKILL.md"),
            )
            .expect("generated skill"),
            body,
            "{surface} keeps the authored body byte for byte"
        );
    }

    let check = Command::new(binary())
        .arg(deployment_request(
            "Check",
            data.path(),
            &[("Psyche", &source)],
            workspace.path(),
            &data.path().join("curriculum.sock"),
        ))
        .output()
        .expect("runtime starts");
    assert!(check.status.success(), "{check:?}");

    let retired = workspace.path().join(".opencode/skills/retired");
    fs::create_dir_all(&retired).expect("retired OpenCode skill directory");
    fs::write(retired.join("SKILL.md"), "retired").expect("retired OpenCode skill");
    let regenerated = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            data.path(),
            &[("Psyche", &source)],
            workspace.path(),
            &data.path().join("curriculum.sock"),
        ))
        .output()
        .expect("runtime starts");
    assert!(regenerated.status.success(), "{regenerated:?}");
    assert!(!retired.exists(), "a retired OpenCode skill is removed");
}

#[test]
fn user_only_catalog_entry_keeps_an_explicit_route_and_is_counted_in_the_receipt() {
    let data = tempdir().expect("data root");
    let source = data.path().join("field");
    let skills = source.join("compensation");
    fs::create_dir_all(&skills).expect("skills directory");
    fs::write(data.path().join("roles.datom"), "Roles.{ [] }").expect("empty role data");
    fs::write(
        version_zero(&skills, "main-flow"),
        "---\ndescription: Main flow\nuser-only: true\n---\n\nExplicit main flow.\n",
    )
    .expect("main flow skill");
    fs::write(version_zero(&skills, "subflow"), "Ordinary subflow.\n").expect("subflow skill");

    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            data.path(),
            &[("Field", &source)],
            workspace.path(),
            &data.path().join("curriculum.sock"),
        ))
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("receipt text")
            .trim(),
        "Generated.{ 2 0 }",
        "the deployment receipt counts the user-only catalog entry"
    );

    assert_eq!(
        fs::read_to_string(
            workspace
                .path()
                .join(".agents/skills/compensation:main-flow:0/agents/openai.yaml"),
        )
        .expect("explicit main-flow policy"),
        "policy:\n  allow_implicit_invocation: false\n"
    );
    assert!(
        workspace
            .path()
            .join(".agents/skills/compensation:main-flow:0/SKILL.md")
            .is_file(),
        "the explicit route has a generated source"
    );
    assert!(
        !workspace
            .path()
            .join(".agents/skills/compensation:subflow:0/agents/openai.yaml")
            .exists(),
        "ordinary skills retain implicit invocation"
    );
}

#[test]
fn freshness_test_generated_module_matches_ethos_file() {
    // Verify that the committed generated.rs matches what ethos-zero would
    // emit from the ethos file. The committed file is rustfmt-formatted,
    // so the emitted output is formatted the same way before comparison.
    use ethos_zero::{Actualizing, File, Generating, Potential};
    let ethos_source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/curriculum-deploy.ethos"
    ))
    .expect("ethos file");
    let concept = Potential::<File>::from(ethos_source)
        .actualize()
        .unwrap_or_else(|_| panic!("ethos file reads"));
    let emitted = concept
        .generate()
        .unwrap_or_else(|_| panic!("ethos file generates"));
    let formatted = format_rust(&emitted).unwrap_or(emitted);
    let committed =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated.rs"))
            .expect("committed generated.rs");
    assert_eq!(
        formatted, committed,
        "src/generated.rs is stale: regenerate with ethos-zero from curriculum-deploy.ethos"
    );
}

fn format_rust(source: &str) -> Option<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut child = Command::new("rustfmt")
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    child.stdin.take()?.write_all(source.as_bytes()).ok()?;
    let output = child.wait_with_output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).ok())?
}

#[test]
#[ignore = "requires the externally owned Curriculum data fixture"]
fn curriculum_roles_round_trip_through_textualize() {
    let data_root = std::env::var("CURRICULUM_TEST_DATA_ROOT")
        .unwrap_or_else(|_| "/external-fixture-not-configured".into());
    let role_path = format!("{data_root}/roles.datom");
    let source = fs::read_to_string(&role_path).expect("roles.datom");

    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let budget = || Budget {
        remaining: 16_384,
        reader: ReaderBudget::try_from(16_384).expect("a constant reader budget"),
        depth: 0,
        maximum_depth: 16_384,
    };
    let mut potential = Potential::<RolesDocument>::from(source);
    let document = potential
        .actualize(&mut budget())
        .expect("read roles document");
    let text_out = document.datomize(vec![]).protosize().textualize();
    let mut potential = Potential::<RolesDocument>::from(text_out.clone());
    let document2 = potential
        .actualize(&mut budget())
        .expect("round-trip roles document");
    assert_eq!(
        text_out,
        document2.datomize(vec![]).protosize().textualize(),
        "round trip changed the roles"
    );
}

#[test]
fn generated_values_archive_and_read_back_with_rkyv() {
    let configuration = curriculum_deploy::generated::Configuration {
        first_string: "/curriculum".into(),
        skill_source_vector: vec![
            curriculum_deploy::generated::SkillSource::Psyche("/psyche".into()),
            curriculum_deploy::generated::SkillSource::Mind("/mind".into()),
            curriculum_deploy::generated::SkillSource::Field("/field".into()),
        ],
        second_string: "/primary".into(),
        third_string: "/run/user/1001/curriculum/curriculum.sock".into(),
    };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configuration).expect("archives");
    let read =
        rkyv::from_bytes::<curriculum_deploy::generated::Configuration, rkyv::rancor::Error>(
            &bytes,
        )
        .expect("reads back");
    assert_eq!(read, configuration);
}
