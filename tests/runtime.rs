use std::{fs, process::Command};

use tempfile::tempdir;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_curriculum-deploy")
}

fn request(operation: &str, workspace: &std::path::Path) -> String {
    let data_root = std::env::var("CURRICULUM_TEST_DATA_ROOT")
        .unwrap_or_else(|_| "/external-fixture-not-configured".into());
    format!("{operation}.{{ {data_root} {} }}", workspace.display())
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
    assert_eq!(agent_skills, 38);
    assert_eq!(claude_skills, 38);
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
    assert!(!workspace.path().join("flows").exists());

    let main = fs::read_to_string(workspace.path().join(".agents/skills/main-flow/SKILL.md"))
        .expect("main-flow role");
    assert!(main.contains("user-only: true"));
    assert!(main.contains("FLOW_ID"));
    assert!(main.contains("FLOW_DIRECTORY"));
    assert!(!main.contains("THREAD_ID"));
    assert!(main.contains("flow-id codex --flows-root"));
    assert!(main.contains("normalized hexadecimal alias"));
    assert!(main.contains("$subflow"));
    assert_eq!(
        fs::read_to_string(
            workspace
                .path()
                .join(".agents/skills/main-flow/agents/openai.yaml"),
        )
        .expect("main-flow invocation policy"),
        "policy:\n  allow_implicit_invocation: false\n"
    );

    let subflow = fs::read_to_string(workspace.path().join(".agents/skills/subflow/SKILL.md"))
        .expect("subflow role");
    assert!(subflow.contains("Pass `FLOW_ID` and `FLOW_DIRECTORY` unchanged"));
    assert!(subflow.contains("Obtain the current `THREAD_ID` from the harness after launch."));
    assert!(subflow.contains("Use `THREAD_ID` only for transcript and evidence provenance."));
    assert!(subflow.contains("Do not create a lane, index entry, or log."));
    assert!(
        !workspace
            .path()
            .join(".agents/skills/subflow/agents/openai.yaml")
            .exists()
    );

    let evidence = fs::read_to_string(
        workspace
            .path()
            .join(".agents/skills/flow-evidence/SKILL.md"),
    )
    .expect("flow-evidence capability");
    assert!(evidence.contains("named tool or flow will consume one"));
    assert!(evidence.contains("main-flow-reserved unique path"));
    assert!(
        !workspace
            .path()
            .join(".agents/skills/flow-evidence/agents/openai.yaml")
            .exists()
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
    assert!(inventory.starts_with("GeneratedRoleOutputs.{ ["));
    assert_eq!(inventory.matches("agents/").count(), 27);

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
        "GeneratedRoleOutputs.{ [ \u{201C}.codex/agents/retired.toml\u{201D} ] }",
    )
    .expect("prior typed inventory");
    let output = Command::new(binary())
        .arg(request("Generate", workspace.path()))
        .output()
        .expect("runtime regenerates");
    assert!(output.status.success(), "{output:?}");
    assert!(!stale.exists());
    assert!(!retired_agent_skill.exists());
    assert!(!retired_claude_skill.exists());
    assert!(
        workspace
            .path()
            .join(".agents/skills/design/agents/openai.yaml")
            .is_file()
    );
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

#[test]
fn legacy_curriculum_request_wrapper_is_accepted() {
    let data = tempdir().expect("data root");
    let skills = data.path().join("skills");
    fs::create_dir_all(&skills).expect("skills directory");
    fs::write(
        data.path().join("roles.datom"),
        "Roles.{ [] [] [] [] [] [] [] [] }",
    )
    .expect("empty role data");
    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(format!(
            "CurriculumRequest.{{ Visualize.{{ {} {} }} }}",
            data.path().display(),
            workspace.path().display()
        ))
        .output()
        .expect("legacy wrapper");
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Visualized."),
        "unexpected output: {stdout}"
    );
}

#[test]
fn skill_conditionals_render_only_for_their_target() {
    let data = tempdir().expect("data root");
    let skills = data.path().join("skills");
    fs::create_dir_all(&skills).expect("skills directory");
    fs::write(
        data.path().join("roles.datom"),
        "Roles.{ [] [] [] [] [] [] [] [] }",
    )
    .expect("empty role data");
    fs::write(
        skills.join("commands.md"),
        "Shared command.\n{% if claude %}\nclaude command\n{% endif %}\n{% if codex %}\ncodex command\n{% endif %}\n",
    )
    .expect("conditional skill");

    let workspace = tempdir().expect("workspace");
    let output = Command::new(binary())
        .arg(format!(
            "Generate.{{ {} {} }}",
            data.path().display(),
            workspace.path().display()
        ))
        .output()
        .expect("runtime starts");
    assert!(output.status.success(), "{output:?}");

    let claude = fs::read_to_string(workspace.path().join(".claude/skills/commands/SKILL.md"))
        .expect("Claude skill");
    assert!(claude.contains("claude command"));
    assert!(!claude.contains("codex command"));

    let codex = fs::read_to_string(workspace.path().join(".agents/skills/commands/SKILL.md"))
        .expect("Codex skill");
    assert!(codex.contains("codex command"));
    assert!(!codex.contains("claude command"));
}

#[test]
fn freshness_test_generated_module_matches_ethos_file() {
    // Verify that the committed generated.rs matches what ethos-zero would
    // emit from the ethos file. The committed file is rustfmt-formatted,
    // so the emitted output is formatted the same way before comparison.
    use ethos_zero::{Actualizing, Emitting, Potential};
    let ethos_source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/curriculum-deploy.ethos"
    ))
    .expect("ethos file");
    let concept = Potential::from(ethos_source.as_str())
        .actualize()
        .expect("ethos file reads");
    let emitted = concept.emit().expect("ethos file emits");
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
