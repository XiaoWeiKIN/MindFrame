use std::{fs, process::Command};
use tempfile::tempdir;

#[test]
fn unsupported_input_fails_before_configuration_or_output() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("out");
    let result = Command::new(env!("CARGO_BIN_EXE_mindframe"))
        .args(["plan", "missing.pdf", "--out"]).arg(&out)
        .args(["--config", "missing.toml"]).output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("single UTF-8 .md"));
    assert!(!out.exists());
}

#[test]
fn empty_source_fails_before_configuration_or_output() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("empty.md");
    fs::write(&source, " \n").unwrap();
    let out = dir.path().join("out");
    let result = Command::new(env!("CARGO_BIN_EXE_mindframe"))
        .arg("plan").arg(source).arg("--out").arg(&out)
        .args(["--config", "missing.toml"]).output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("Markdown is empty"));
    assert!(!out.exists());
}

#[test]
fn missing_credential_never_creates_a_project() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.md");
    fs::write(&source, "# Test\nMy own knowledge.").unwrap();
    let config = dir.path().join("config.toml");
    fs::write(&config, "[llm]\nbase_url='https://example.com/v1'\nmodel='test'\napi_key_env='MINDFRAME_INTENTIONALLY_UNSET'\n").unwrap();
    let out = dir.path().join("out");
    let result = Command::new(env!("CARGO_BIN_EXE_mindframe"))
        .env_remove("MINDFRAME_INTENTIONALLY_UNSET")
        .arg("plan").arg(source).arg("--out").arg(&out)
        .arg("--config").arg(config).output().unwrap();
    assert!(!result.status.success());
    assert!(!out.exists());
}

#[test]
fn schemas_are_generated_from_the_domain_types() {
    let dir = tempdir().unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_mindframe"))
        .args(["schema", "--out"]).arg(dir.path()).output().unwrap();
    assert!(result.status.success());
    let schema: serde_json::Value = serde_json::from_str(&fs::read_to_string(dir.path().join("storyboard.schema.json")).unwrap()).unwrap();
    assert_eq!(schema["additionalProperties"], false);
    assert!(schema["properties"]["scenes"].is_object());
}
