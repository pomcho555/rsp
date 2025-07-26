use rsp::peeler::Peeler;
use serde_yaml::Value;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_empty_configmap() {
    let peeler = Peeler::new();

    let yaml_content = r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: empty-config
data: {}
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    let result = peeler.process_yaml_value(&mut yaml_value);
    assert!(result.is_ok());
}

#[test]
fn test_configmap_without_data() {
    let peeler = Peeler::new();

    let yaml_content = r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: no-data-config
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    let result = peeler.process_yaml_value(&mut yaml_value);
    assert!(result.is_ok());
}

#[test]
fn test_configmap_with_non_string_data() {
    let peeler = Peeler::new();

    let yaml_content = r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: mixed-data-config
data:
  number-value: 42
  boolean-value: true
  config.json: "{\"hello\":\"world\"}"
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    let result = peeler.process_yaml_value(&mut yaml_value);
    assert!(result.is_ok());

    // Check that only string values with proper extensions are processed
    if let Value::Mapping(map) = &yaml_value {
        if let Some(Value::Mapping(data)) = map.get(Value::String("data".to_string())) {
            // Number and boolean should remain unchanged
            assert!(matches!(
                data.get(Value::String("number-value".to_string())),
                Some(Value::Number(_))
            ));
            assert!(matches!(
                data.get(Value::String("boolean-value".to_string())),
                Some(Value::Bool(_))
            ));
        }
    }
}

#[test]
fn test_deeply_nested_escapes() {
    let peeler = Peeler::new();

    let complex_escaped = r#"{\n  \"outer\": {\n    \"inner\": \"value\\nwith\\nnewlines\",\n    \"quoted\": \"say \\\"hello\\\"\"\n  }\n}"#;
    let result = peeler.unescape_string(complex_escaped).unwrap();

    // Test that the basic structure is unescaped correctly
    assert!(result.contains("outer"));
    assert!(result.contains("inner"));
    assert!(result.contains("quoted"));
    assert!(result.contains("hello"));
    // Test that newlines are properly converted from \n to actual newlines
    assert!(result.contains('\n'));
}

#[test]
fn test_yaml_with_special_characters() {
    let peeler = Peeler::new();

    let yaml_content = r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: special-chars-config
data:
  config.json: "{\"unicode\":\"\\u00e9\\u00e8\",\n\"symbols\":\"@#$%^&*()\"}"
  weird-file-name.yaml: "key: value\\nwith: symbols"
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    let result = peeler.process_yaml_value(&mut yaml_value);
    assert!(result.is_ok());
}

#[test]
fn test_very_large_string() {
    let peeler = Peeler::new();

    // Create a large string with many lines
    let large_content = (0..1000)
        .map(|i| format!("line{}: this is line number {}", i, i))
        .collect::<Vec<_>>()
        .join("\\n");

    let result = peeler.unescape_string(&large_content).unwrap();
    assert!(result.lines().count() == 1000);
    assert!(result.contains("line0: this is line number 0"));
    assert!(result.contains("line999: this is line number 999"));
}

#[test]
fn test_malformed_yaml_structure() {
    let peeler = Peeler::new();

    // YAML that parses but has unexpected structure
    let yaml_content = r#"
not_a_configmap:
  kind: ConfigMap
  data:
    config.json: "{\"hello\":\"world\"}"
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    let result = peeler.process_yaml_value(&mut yaml_value);
    assert!(result.is_ok());

    // Should not process data since it's not a proper ConfigMap
    if let Value::Mapping(map) = &yaml_value {
        if let Some(Value::Mapping(inner)) = map.get(Value::String("not_a_configmap".to_string())) {
            if let Some(Value::Mapping(_data)) = inner.get(Value::String("data".to_string())) {
                if let Some(Value::String(json_content)) =
                    _data.get(Value::String("config.json".to_string()))
                {
                    assert_eq!(json_content, "{\"hello\":\"world\"}"); // Should remain escaped
                }
            }
        }
    }
}

#[test]
fn test_empty_file_processing() {
    let peeler = Peeler::new();

    let mut input_file = NamedTempFile::new().unwrap();
    write!(input_file, "").unwrap();
    let input_path = input_file.path().to_str().unwrap();

    let result = peeler.peel_file(input_path, None);
    // Empty file should result in YAML parsing error or other error
    assert!(result.is_err());
}

#[test]
fn test_binary_file_processing() {
    let peeler = Peeler::new();

    let mut input_file = NamedTempFile::new().unwrap();
    // Write some binary data
    input_file.write_all(&[0xFF, 0xFE, 0xFD, 0xFC]).unwrap();
    let input_path = input_file.path().to_str().unwrap();

    let result = peeler.peel_file(input_path, None);
    assert!(result.is_err()); // Should fail to parse as YAML or UTF-8
}

#[test]
fn test_extremely_nested_escapes() {
    let peeler = Peeler::new();

    // Test case with multiple levels of escaping
    let nested_escaped = r#"\\n\\t\\r\\\"\\\\"#;
    let result = peeler.unescape_string(nested_escaped).unwrap();
    assert_eq!(result, "\\n\\t\\r\\\"\\\\");
}

#[test]
fn test_unicode_in_escaped_strings() {
    let peeler = Peeler::new();

    let unicode_content = "Hello\\nWorld\\n🌍\\n世界";
    let result = peeler.unescape_string(unicode_content).unwrap();
    assert!(result.contains("Hello\nWorld\n🌍\n世界"));
}

#[test]
fn test_mixed_file_extensions() {
    let peeler = Peeler::new();

    let yaml_content = r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: mixed-extensions
data:
  config.json: "{\"json\":\"content\"}"
  config.yaml: "yaml: content"
  config.yml: "yml: content" 
  config.toml: "toml = \"content\""
  config.txt: "should not process"
  config.xml: "<should>not process</should>"
  no-extension: "should not process"
"#;

    let mut yaml_value: Value = serde_yaml::from_str(yaml_content).unwrap();
    peeler.process_yaml_value(&mut yaml_value).unwrap();

    if let Value::Mapping(map) = &yaml_value {
        if let Some(Value::Mapping(_data)) = map.get(Value::String("data".to_string())) {
            // These should be processed (unescaped)
            assert!(peeler.should_process_key("config.json"));
            assert!(peeler.should_process_key("config.yaml"));
            assert!(peeler.should_process_key("config.yml"));
            assert!(peeler.should_process_key("config.toml"));

            // These should not be processed
            assert!(!peeler.should_process_key("config.txt"));
            assert!(!peeler.should_process_key("config.xml"));
            assert!(!peeler.should_process_key("no-extension"));
        }
    }
}
