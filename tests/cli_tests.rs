use std::process::Command;
use std::fs;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_cli_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Raw String Peeler"));
    assert!(stdout.contains("peel"));
    assert!(stdout.contains("--help"));
    assert!(stdout.contains("--version"));
}

#[test]
fn test_cli_version_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("rsp 0.1.0"));
}

#[test]
fn test_cli_peel_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "peel", "--help"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Peel raw strings from YAML files"));
    assert!(stdout.contains("<FILE>"));
    assert!(stdout.contains("--output"));
}

#[test]
fn test_cli_peel_success() {
    let yaml_content = r#"apiVersion: v1
kind: ConfigMap
metadata:
  name: test-config
data:
  config.json: "{\"hello\":\"world\",\n\"foo\":\"bar\"}"
"#;
    
    // Create temporary input file
    let mut input_file = NamedTempFile::new().unwrap();
    write!(input_file, "{}", yaml_content).unwrap();
    let input_path = input_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "peel", input_path])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("config.json: |"));
    assert!(stdout.contains("  {\"hello\":\"world\","));
    assert!(stdout.contains("  \"foo\":\"bar\"}"));
}

#[test]
fn test_cli_peel_with_output_file() {
    let yaml_content = r#"apiVersion: v1
kind: ConfigMap
metadata:
  name: test-config
data:
  config.json: "{\"hello\":\"world\",\n\"foo\":\"bar\"}"
"#;
    
    // Create temporary input file
    let mut input_file = NamedTempFile::new().unwrap();
    write!(input_file, "{}", yaml_content).unwrap();
    let input_path = input_file.path().to_str().unwrap();
    
    // Create temporary output file
    let output_file = NamedTempFile::new().unwrap();
    let output_path = output_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "peel", input_path, "-o", output_path])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    // Check that output file was created and contains expected content
    let file_content = fs::read_to_string(output_path).unwrap();
    assert!(file_content.contains("config.json: |"));
    assert!(file_content.contains("  {\"hello\":\"world\","));
}

#[test]
fn test_cli_peel_file_not_found() {
    let output = Command::new("cargo")
        .args(&["run", "--", "peel", "nonexistent_file.yaml"])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    // The error could be in stderr or stdout depending on implementation
    let combined_output = format!("{}{}", stderr, String::from_utf8(output.stdout).unwrap_or_default());
    assert!(combined_output.contains("File not found") || combined_output.contains("No such file") || combined_output.contains("nonexistent_file.yaml"));
}

#[test]
fn test_cli_peel_invalid_yaml() {
    let invalid_yaml = "invalid: yaml: content: [unclosed";
    
    let mut input_file = NamedTempFile::new().unwrap();
    write!(input_file, "{}", invalid_yaml).unwrap();
    let input_path = input_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "peel", input_path])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
}

#[test]
fn test_cli_no_command() {
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("No command provided") || stderr.contains("help"));
}

#[test]
fn test_cli_peel_missing_argument() {
    let output = Command::new("cargo")
        .args(&["run", "--", "peel"])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("required") || stderr.contains("FILE"));
}