use std::{fs, process::Command};

use tempfile::tempdir;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_curriculum-deploy")
}

fn request(operation: &str, workspace: &std::path::Path) -> String {
    let data_root = std::env::var("CURRICULUM_TEST_DATA_ROOT")
        .unwrap_or_else(|_| "/external-fixture-not-configured".into());
    format!(
        "CurriculumRequest.{{{operation}.{{{data_root} {}}}}}",
        workspace.display()
    )
}

#[test]
#[ignore = "requires the externally owned Curriculum data fixture"]
fn external_data_generates_skills_roles_and_a_typed_cleanup_inventory() {
    let workspace = tempdir().expect("workspace");
    let unrelated = tempdir().expect("unrelated current directory");
    let output = Command::new(binary())
        .arg(request("Generate", workspace.path()))
        .current_dir(unrelated.path())
        .env_clear()
        .env("SKILLS_SOURCE_ROOT", "/poison")
        .env("SKILLS_WORKSPACE_ROOT", "/poison")
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");

    let agent_skills = fs::read_dir(workspace.path().join(".agents/skills"))
        .expect("agent skills")
        .count();
    let claude_skills = fs::read_dir(workspace.path().join(".claude/skills"))
        .expect("claude skills")
        .count();
    assert_eq!(agent_skills, 35);
    assert_eq!(claude_skills, 35);
    assert!(
        workspace
            .path()
            .join(".claude/skills/design/SKILL.md")
            .is_file()
    );
    assert!(
        workspace
            .path()
            .join(".claude/skills/realization/SKILL.md")
            .is_file()
    );

    let roles: Vec<_> = [".claude/agents", ".codex/agents", ".pi/agents"]
        .into_iter()
        .flat_map(|relative| fs::read_dir(workspace.path().join(relative)).expect("role directory"))
        .collect();
    assert_eq!(roles.len(), 27);
    let codex = fs::read_to_string(workspace.path().join(".codex/agents/worker.toml"))
        .expect("worker alias");
    assert!(codex.contains("gpt-5.6-terra"));
    assert!(codex.contains("Do not reload a complete pasted skill"));
    let claude = fs::read_to_string(workspace.path().join(".claude/agents/write-demanding.md"))
        .expect("Claude role");
    assert!(claude.contains("claude-opus-4-6[1m]"));
    assert!(claude.contains("The brief is your authority"));
    let inventory =
        fs::read_to_string(workspace.path().join("skills/generated-role-outputs.datom"))
            .expect("typed cleanup inventory");
    assert!(inventory.starts_with("GeneratedRoleOutputs.{["));
    assert_eq!(inventory.matches("agents/").count(), 27);

    let stale = workspace.path().join(".codex/agents/retired.toml");
    fs::write(&stale, "retired").expect("stale role");
    fs::write(
        workspace.path().join("skills/generated-role-outputs.datom"),
        "GeneratedRoleOutputs.{[.codex/agents/retired.toml]}",
    )
    .expect("prior typed inventory");
    let output = Command::new(binary())
        .arg(request("Generate", workspace.path()))
        .output()
        .expect("runtime regenerates");
    assert!(output.status.success(), "{output:?}");
    assert!(!stale.exists());
    let output = Command::new(binary())
        .arg(request("Check", workspace.path()))
        .output()
        .expect("runtime checks generated output");
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn cli_accepts_only_one_inline_object() {
    let workspace = tempdir().expect("workspace");
    let valid = request("Visualize", workspace.path());
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
