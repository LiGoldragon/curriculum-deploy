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
    fs::write(
        root.join("roles.datom"),
        "Roles.{ [] [] [] [] [] [] [] [] }",
    )
    .expect("empty role data");
}

fn source_with(root: &Path, name: &str, skills: &[(&str, &str)]) -> std::path::PathBuf {
    let directory = root.join(name);
    fs::create_dir_all(&directory).expect("source directory");
    for (skill, body) in skills {
        fs::write(directory.join(format!("{skill}.md")), body).expect("skill source");
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
    source_with(curriculum.path(), "skills", &[("stray", "Not declared.\n")]);
    let repositories = tempdir().expect("source repositories");
    let psyche = source_with(
        repositories.path(),
        "psyche-skills",
        &[("vision", "Psyche skill.\n")],
    );
    let mind = source_with(
        repositories.path(),
        "mind-skills",
        &[
            ("design", "Mind skill.\n"),
            ("testing", "Mind test skill.\n"),
        ],
    );
    let field = source_with(
        repositories.path(),
        "field-skills",
        &[(
            "operation",
            "---\ndescription: Field\nuser-only: true\n---\n\nField skill.\n",
        )],
    );
    // A non-Markdown file in a source is not a skill.
    fs::write(field.join("notes.txt"), "not a skill").expect("non-skill file");

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

    let expected: BTreeSet<String> = ["design", "operation", "testing", "vision"]
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
        fs::read_to_string(workspace.path().join(".claude/skills/vision/SKILL.md"))
            .expect("psyche skill"),
        "Psyche skill.\n"
    );
    assert_eq!(
        fs::read_to_string(workspace.path().join(".claude/skills/operation/SKILL.md"))
            .expect("field skill"),
        "---\ndescription: Field\ndisable-model-invocation: true\n---\n\nField skill.\n"
    );
    assert!(
        workspace
            .path()
            .join(".agents/skills/operation/agents/openai.yaml")
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
    let psyche = source_with(
        repositories.path(),
        "psyche-skills",
        &[("alpha", "Psyche alpha.\n"), ("shared", "Psyche shared.\n")],
    );
    let field = source_with(
        repositories.path(),
        "field-skills",
        &[("shared", "Field shared.\n"), ("omega", "Field omega.\n")],
    );

    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(deployment_request(
            "Generate",
            curriculum.path(),
            &[("Psyche", &psyche), ("Field", &field)],
            workspace.path(),
        ))
        .output()
        .expect("runtime starts");
    assert!(!output.status.success(), "{output:?}");
    let fault = String::from_utf8(output.stderr).expect("fault text");
    assert_eq!(
        fault.trim(),
        format!(
            "skill shared is defined by two sources: Psyche {} and Field {}",
            psyche.join("shared.md").display(),
            field.join("shared.md").display()
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
