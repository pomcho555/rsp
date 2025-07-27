use rsp_cli::error::RspError;
use rsp_cli::peeler::Peeler;
use serde_yaml::Value;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_unescape_string_normal_cases() {
    let peeler = Peeler::new();

    // Test newline unescaping
    let result = peeler.unescape_string("hello\\nworld").unwrap();
    assert_eq!(result, "hello\nworld");

    // Test tab unescaping
    let result = peeler.unescape_string("hello\\tworld").unwrap();
    assert_eq!(result, "hello\tworld");

    // Test quote unescaping
    let result = peeler.unescape_string("hello\\\"world").unwrap();
    assert_eq!(result, "hello\"world");

    // Test backslash unescaping
    let result = peeler.unescape_string("hello\\\\world").unwrap();
    assert_eq!(result, "hello\\world");

    // Test carriage return unescaping
    let result = peeler.unescape_string("hello\\rworld").unwrap();
    assert_eq!(result, "hello\rworld");

    // Test multiple escapes
    let result = peeler
        .unescape_string("line1\\nline2\\tindented\\\"quoted\\\"")
        .unwrap();
    assert_eq!(result, "line1\nline2\tindented\"quoted\"");
}

#[test]
fn test_unescape_string_edge_cases() {
    let peeler = Peeler::new();

    // Test empty string
    let result = peeler.unescape_string("").unwrap();
    assert_eq!(result, "");

    // Test string with no escapes
    let result = peeler.unescape_string("hello world").unwrap();
    assert_eq!(result, "hello world");

    // Test backslash at end
    let result = peeler.unescape_string("hello\\").unwrap();
    assert_eq!(result, "hello\\");

    // Test unknown escape sequence
    let result = peeler.unescape_string("hello\\zworld").unwrap();
    assert_eq!(result, "hello\\zworld");

    // Test multiple consecutive backslashes
    let result = peeler.unescape_string("hello\\\\\\\\world").unwrap();
    assert_eq!(result, "hello\\\\world");
}

#[test]
fn test_should_process_key() {
    let peeler = Peeler::new();

    // Test valid extensions
    assert!(peeler.should_process_key("config.yaml"));
    assert!(peeler.should_process_key("config.yml"));
    assert!(peeler.should_process_key("config.json"));
    assert!(peeler.should_process_key("config.toml"));

    // Test invalid extensions
    assert!(!peeler.should_process_key("config.txt"));
    assert!(!peeler.should_process_key("config.xml"));
    assert!(!peeler.should_process_key("config"));
    assert!(!peeler.should_process_key(""));

    // Test complex filenames
    assert!(peeler.should_process_key("my-app-config.yaml"));
    assert!(peeler.should_process_key("database.config.json"));
    assert!(!peeler.should_process_key("config.yaml.backup"));
}

#[test]
fn test_process_configmap_normal() {
    let peeler = Peeler::new();

    let yaml_content = r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: test-config
data:
  config.json: "{\"hello\":\"world\",\n\"foo\":\"bar\"}"
  config.yaml: "key: value\nanother: test"
  normal-file: "This should not be processed"
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    peeler.process_yaml_value(&mut yaml_value).unwrap();

    // Verify the processing worked
    if let Value::Mapping(map) = &yaml_value {
        if let Some(Value::Mapping(data)) = map.get(Value::String("data".to_string())) {
            if let Some(Value::String(json_content)) =
                data.get(Value::String("config.json".to_string()))
            {
                assert!(json_content.contains("\"hello\":\"world\""));
                assert!(json_content.contains('\n'));
            } else {
                panic!("config.json not found or not a string");
            }

            if let Some(Value::String(normal_content)) =
                data.get(Value::String("normal-file".to_string()))
            {
                assert_eq!(normal_content, "This should not be processed");
            } else {
                panic!("normal-file not found or not a string");
            }
        } else {
            panic!("data section not found");
        }
    } else {
        panic!("Root is not a mapping");
    }
}

#[test]
fn test_process_yaml_invalid_format() {
    let peeler = Peeler::new();

    // Test with non-mapping root
    let mut yaml_value = Value::String("not a mapping".to_string());
    let result = peeler.process_yaml_value(&mut yaml_value);
    assert!(matches!(result, Err(RspError::InvalidFormat(_))));

    // Test with array root
    let mut yaml_value = Value::Sequence(vec![]);
    let result = peeler.process_yaml_value(&mut yaml_value);
    assert!(matches!(result, Err(RspError::InvalidFormat(_))));
}

#[test]
fn test_process_non_configmap() {
    let peeler = Peeler::new();

    let yaml_content = r#"
apiVersion: v1
kind: Secret
metadata:
  name: test-secret
data:
  config.json: "{\"hello\":\"world\"}"
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    peeler.process_yaml_value(&mut yaml_value).unwrap();

    // Verify that Secret data is not processed (only ConfigMap should be processed)
    if let Value::Mapping(map) = &yaml_value {
        if let Some(Value::Mapping(data)) = map.get(Value::String("data".to_string())) {
            if let Some(Value::String(json_content)) =
                data.get(Value::String("config.json".to_string()))
            {
                assert_eq!(json_content, "{\"hello\":\"world\"}"); // Should remain escaped
            }
        }
    }
}

#[test]
fn test_serialize_yaml_with_pipes() {
    let peeler = Peeler::new();

    let yaml_content = r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: test-config
data:
  config.json: "{\"hello\":\"world\",\n\"foo\":\"bar\"}"
  simple: "no newlines here"
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    peeler.process_yaml_value(&mut yaml_value).unwrap();

    let result = peeler.serialize_yaml_with_pipes(&yaml_value).unwrap();

    // Check that multiline content uses pipe syntax
    assert!(result.contains("config.json: |"));
    assert!(result.contains("  {\"hello\":\"world\","));
    assert!(result.contains("  \"foo\":\"bar\"}"));

    // Check that simple content doesn't use pipe syntax
    assert!(result.contains("simple: no newlines here"));
    assert!(!result.contains("simple: |"));
}

#[cfg(test)]
mod file_tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_peel_file_success() {
        let peeler = Peeler::new();

        let yaml_content = r#"apiVersion: v1
kind: ConfigMap
metadata:
  name: test-config
data:
  config.json: "{\"hello\":\"world\",\n\"foo\":\"bar\"}"
"#;

        // Create temporary input file
        let mut input_file = NamedTempFile::new().unwrap();
        write!(input_file, "{yaml_content}").unwrap();
        let input_path = input_file.path().to_str().unwrap();

        // Create temporary output file
        let output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_str().unwrap();

        // Test processing
        let result = peeler.peel_file(input_path, Some(&output_path.to_string()));
        assert!(result.is_ok());

        // Verify output file content
        let output_content = fs::read_to_string(output_path).unwrap();
        assert!(output_content.contains("config.json: |"));
        assert!(output_content.contains("  {\"hello\":\"world\","));
    }

    #[test]
    fn test_peel_file_not_found() {
        let peeler = Peeler::new();

        let result = peeler.peel_file("nonexistent_file.yaml", None);
        assert!(matches!(result, Err(RspError::FileNotFound(_))));
    }

    #[test]
    fn test_peel_file_invalid_yaml() {
        let peeler = Peeler::new();

        let invalid_yaml = "invalid: yaml: content: [unclosed";

        let mut input_file = NamedTempFile::new().unwrap();
        write!(input_file, "{invalid_yaml}").unwrap();
        let input_path = input_file.path().to_str().unwrap();

        let result = peeler.peel_file(input_path, None);
        assert!(matches!(result, Err(RspError::Yaml(_))));
    }
}
