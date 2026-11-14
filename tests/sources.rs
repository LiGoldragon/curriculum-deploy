use std::{collections::BTreeSet, fs, path::Path, process::Command};

use tempfile::tempdir;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_curriculum-deploy")
}

fn deployment_request(
    operation: &str,
    curriculum: &Path,
    sources: &[(&str, &Path)],
    workspace: &Path,
) -> String {
    let sources = sources
        .iter()
        .map(|(aspect, directory)| format!("{aspect}.«{}»", directory.display()))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "{operation}.{{ «{}» [ {sources} ] «{}» }}",
        curriculum.display(),
        workspace.display()
    )
}

fn curriculum_with_empty_roles(root: &Path) {
    fs::write(root.join("roles.datom"), "Roles.{ [] }").expect("empty role data");
}

/// An aspect repository holding each skill as version 0 of its subject:
/// `<kind>/<subject>/0.md`.
fn source_with(root: &Path, name: &str, kind: &str, skills: &[(&str, &str)]) -> std::path::PathBuf {
    let directory = root.join(name);
    for (skill, body) in skills {
        let subject = directory.join(kind).join(skill);
        fs::create_dir_all(&subject).expect("subject directory");
        fs::write(subject.join("0.md"), body).expect("skill source");
    }
    directory
}

fn generated_skill_names(workspace: &Path, surface: &str) -> BTreeSet<String> {
    fs::read_dir(workspace.join(surface))
        .expect("generated skill surface")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join("SKILL.md").is_file())
        .map(|entry| entry.file_name().into_string().expect("UTF-8 skill name"))
        .collect()
}

#[test]
fn skills_from_three_aspect_sources_deploy_as_one_catalog() {
    let curriculum = tempdir().expect("curriculum root");
    curriculum_with_empty_roles(curriculum.path());
    // Curriculum contributes role data only; a skills directory left in it is
    // not a declared source and is not deployed.
    source_with(
        curriculum.path(),
        "skills",
        "vision",
        &[("stray", "Not declared.\n")],
    );
    let repositories = tempdir().expect("source repositories");
    let psyche = source_with(
        repositories.path(),
        "psyche-skills",
        "vision",
        &[("direction", "Psyche skill.\n")],
    );
    let mind = source_with(
        repositories.path(),
        "mind-skills",
        "knowledge",
        &[
            ("design", "Mind skill.\n"),
            ("testing", "Mind test skill.\n"),
        ],
    );
    let field = source_with(
        repositories.path(),
        "field-skills",
        "compensation",
        &[(
            "operation",
            "---\ndescription: Field\nuser-only: true\n---\n\nField skill.\n",
        )],
    );
    // A non-Markdown file in a source is not a skill.
    fs::write(field.join("compensation/notes.txt"), "not a skill").expect("non-skill file");
    fs::write(psyche.join("README.md"), "repository documentation").expect("root documentation");
    fs::create_dir_all(mind.join("evidence")).expect("non-module directory");
    fs::write(mind.join("evidence/report.md"), "not a module").expect("evidence report");
    fs::create_dir_all(mind.join("inquiry")).expect("retired module directory");
    fs::write(mind.join("inquiry/retired.md"), "not loaded").expect("retired module");

    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            curriculum.path(),
            &[("Psyche", &psyche), ("Mind", &mind), ("Field", &field)],
            workspace.path(),
        ))
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).expect("receipt").trim(),
        "Generated.{ 4 0 }"
    );

    let expected: BTreeSet<String> = [
        "compensation:operation:0",
        "knowledge:design:0",
        "knowledge:testing:0",
        "vision:direction:0",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    assert_eq!(
        generated_skill_names(workspace.path(), ".agents/skills"),
        expected
    );
    assert_eq!(
        generated_skill_names(workspace.path(), ".claude/skills"),
        expected
    );
    assert_eq!(
        fs::read_to_string(
            workspace
                .path()
                .join(".claude/skills/vision:direction:0/SKILL.md"),
        )
        .expect("psyche skill"),
        "Psyche skill.\n"
    );
    assert_eq!(
        fs::read_to_string(
            workspace
                .path()
                .join(".claude/skills/compensation:operation:0/SKILL.md"),
        )
        .expect("field skill"),
        "---\ndescription: Field\ndisable-model-invocation: true\n---\n\nField skill.\n"
    );
    assert!(
        workspace
            .path()
            .join(".agents/skills/compensation:operation:0/agents/openai.yaml")
            .is_file()
    );

    let output = Command::new(binary())
        .arg(deployment_request(
            "Check",
            curriculum.path(),
            &[("Field", &field), ("Mind", &mind), ("Psyche", &psyche)],
            workspace.path(),
        ))
        .output()
        .expect("runtime checks");
    assert!(
        output.status.success(),
        "declaration order does not change the output: {output:?}"
    );
}

#[test]
fn a_skill_defined_by_two_sources_is_refused_and_nothing_is_written() {
    let curriculum = tempdir().expect("curriculum root");
    curriculum_with_empty_roles(curriculum.path());
    let repositories = tempdir().expect("source repositories");
    let mind = source_with(
        repositories.path(),
        "mind-skills",
        "role",
        &[("alpha", "Psyche alpha.\n"), ("shared", "Psyche shared.\n")],
    );
    let field = source_with(
        repositories.path(),
        "field-skills",
        "role",
        &[("shared", "Field shared.\n"), ("omega", "Field omega.\n")],
    );

    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            curriculum.path(),
            &[("Mind", &mind), ("Field", &field)],
            workspace.path(),
        ))
        .output()
        .expect("runtime starts");
    assert!(!output.status.success(), "{output:?}");
    let fault = String::from_utf8(output.stderr).expect("fault text");
    assert_eq!(
        fault.trim(),
        format!(
            "skill role:shared:0 is defined by two sources: Mind {} and Field {}",
            mind.join("role/shared/0.md").display(),
            field.join("role/shared/0.md").display()
        )
    );
    assert!(output.stdout.is_empty());
    assert_eq!(
        fs::read_dir(workspace.path()).expect("workspace").count(),
        0,
        "a refused deployment writes nothing"
    );
}

#[test]
fn a_missing_source_directory_is_refused() {
    let curriculum = tempdir().expect("curriculum root");
    curriculum_with_empty_roles(curriculum.path());
    let missing = curriculum.path().join("no-such-source");
    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Visualize",
            curriculum.path(),
            &[("Mind", &missing)],
            workspace.path(),
        ))
        .output()
        .expect("runtime starts");
    assert!(!output.status.success(), "{output:?}");
    assert!(
        String::from_utf8(output.stderr)
            .expect("fault text")
            .starts_with(&format!("read {}:", missing.display()))
    );
}

#[test]
fn an_undeclared_aspect_is_refused() {
    let curriculum = tempdir().expect("curriculum root");
    curriculum_with_empty_roles(curriculum.path());
    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Visualize",
            curriculum.path(),
            &[("Spirit", curriculum.path())],
            workspace.path(),
        ))
        .output()
        .expect("runtime starts");
    assert!(!output.status.success(), "{output:?}");
}

#[test]
fn roles_name_their_modules_by_colon_ids() {
    let curriculum = tempdir().expect("curriculum root");
    fs::write(
        curriculum.path().join("roles.datom"),
        "Roles.{ [ { relay ClaudeAgent sonnet High Unrestricted role:relay-description:0 [ spirit:core:0 role:relay:0 ] Explicit.[ Read Bash ] Only.[ { Psyche core Tertiary } ] } ] }",
    )
    .expect("role data");
    let repositories = tempdir().expect("source repositories");
    let psyche = source_with(
        repositories.path(),
        "psyche-skills",
        "spirit",
        &[("core", "---\ndescription: Core\n---\n\nCORE_BODY\n")],
    );
    let field = source_with(
        repositories.path(),
        "field-skills",
        "role",
        &[
            ("relay-description", "Relay the comments.\n"),
            ("relay", "RELAY_BODY\n"),
        ],
    );

    let workspace = tempdir().expect("workspace");
    let sources = [("Psyche", psyche.as_path()), ("Field", field.as_path())];
    let output = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            curriculum.path(),
            &sources,
            workspace.path(),
        ))
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).expect("receipt").trim(),
        "Generated.{ 3 1 }"
    );
    assert!(
        workspace
            .path()
            .join(".claude/skills/spirit:core:0/SKILL.md")
            .is_file()
    );
    let agent = fs::read_to_string(workspace.path().join(".claude/agents/relay.md"))
        .expect("Claude role packet");
    assert!(agent.starts_with(
        "---\nname: relay\ndescription: \"Relay the comments.\"\nmodel: \"sonnet\"\neffort: high\ntools: [Read, Bash]\n---\n"
    ));
    assert!(agent.contains("Module IDs: spirit:core:0, role:relay:0\n"));
    assert!(
        agent.contains(
            "<!-- module: spirit:core:0 -->\nCORE_BODY\n<!-- end-module: spirit:core:0 -->"
        )
    );
    assert!(
        agent.contains(
            "<!-- module: role:relay:0 -->\nRELAY_BODY\n<!-- end-module: role:relay:0 -->"
        )
    );

    let output = Command::new(binary())
        .arg(deployment_request(
            "Check",
            curriculum.path(),
            &sources,
            workspace.path(),
        ))
        .output()
        .expect("runtime checks");
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn each_version_of_a_subject_is_a_module_and_nothing_else_is() {
    let curriculum = tempdir().expect("curriculum root");
    curriculum_with_empty_roles(curriculum.path());
    let repositories = tempdir().expect("source repositories");
    let psyche = repositories.path().join("psyche-belief");
    let core = psyche.join("spirit/core");
    fs::create_dir_all(&core).expect("subject directory");
    fs::write(core.join("0.md"), "Version zero.\n").expect("version 0");
    fs::write(core.join("1.md"), "Version one.\n").expect("version 1");
    // Not versions: a leading zero, a word, a file of another kind.
    fs::write(core.join("01.md"), "Not canonical.\n").expect("leading zero");
    fs::write(core.join("draft.md"), "Not a version.\n").expect("word stem");
    fs::write(core.join("0.txt"), "Not Markdown.\n").expect("other extension");
    // A file beside the subjects, in the layout before versions, is not a module.
    fs::write(psyche.join("spirit/flat.md"), "Flat.\n").expect("flat file");

    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            curriculum.path(),
            &[("Psyche", &psyche)],
            workspace.path(),
        ))
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");
    let expected: BTreeSet<String> = ["spirit:core:0", "spirit:core:1"]
        .into_iter()
        .map(String::from)
        .collect();
    assert_eq!(
        generated_skill_names(workspace.path(), ".claude/skills"),
        expected
    );
    assert_eq!(
        fs::read_to_string(
            workspace
                .path()
                .join(".claude/skills/spirit:core:1/SKILL.md")
        )
        .expect("version 1"),
        "Version one.\n"
    );
}
