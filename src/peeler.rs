use crate::error::RspError;
use serde_yaml::{Mapping, Value};
use std::fs;
use std::io::{self, Read};

pub struct Peeler;

impl Default for Peeler {
    fn default() -> Self {
        Self::new()
    }
}

impl Peeler {
    pub fn new() -> Self {
        Self
    }

    pub fn peel_file(
        &self,
        input_file: &str,
        output_file: Option<&String>,
    ) -> Result<(), RspError> {
        let content = fs::read_to_string(input_file)
            .map_err(|_| RspError::FileNotFound(input_file.to_string()))?;

        self.peel_content(&content, output_file)
    }

    pub fn peel_stdin(&self, output_file: Option<&String>) -> Result<(), RspError> {
        let mut content = String::new();
        io::stdin()
            .read_to_string(&mut content)
            .map_err(|e| RspError::Processing(format!("Failed to read from stdin: {e}")))?;

        self.peel_content(&content, output_file)
    }

    fn peel_content(&self, content: &str, output_file: Option<&String>) -> Result<(), RspError> {
        let mut yaml_value: Value = serde_yaml::from_str(content)?;

        self.process_yaml_value(&mut yaml_value)?;

        let output = self.serialize_yaml_with_pipes(&yaml_value)?;

        match output_file {
            Some(file_path) => {
                fs::write(file_path, output)?;
                println!("Output written to {file_path}");
            }
            None => {
                print!("{output}");
            }
        }

        Ok(())
    }

    pub fn process_yaml_value(&self, value: &mut Value) -> Result<(), RspError> {
        match value {
            Value::Mapping(map) => {
                self.process_configmap(map)?;
            }
            _ => {
                return Err(RspError::InvalidFormat(
                    "Expected YAML mapping at root level".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn process_configmap(&self, map: &mut Mapping) -> Result<(), RspError> {
        if let Some(Value::String(kind)) = map.get(Value::String("kind".to_string())) {
            if kind == "ConfigMap" {
                if let Some(Value::Mapping(data_map)) =
                    map.get_mut(Value::String("data".to_string()))
                {
                    self.process_data_section(data_map)?;
                }
            }
        }
        Ok(())
    }

    fn process_data_section(&self, data_map: &mut Mapping) -> Result<(), RspError> {
        let keys_to_process: Vec<_> = data_map
            .iter()
            .filter_map(|(key, value)| {
                if let (Value::String(key_str), Value::String(value_str)) = (key, value) {
                    if self.should_process_key(key_str) {
                        Some((key.clone(), value_str.clone()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for (key, value_str) in keys_to_process {
            let processed = self.process_raw_string(&value_str)?;
            data_map.insert(key, Value::String(processed));
        }

        Ok(())
    }

    pub fn should_process_key(&self, key: &str) -> bool {
        key.ends_with(".yaml")
            || key.ends_with(".yml")
            || key.ends_with(".json")
            || key.ends_with(".toml")
    }

    fn process_raw_string(&self, raw_string: &str) -> Result<String, RspError> {
        let unescaped = self.unescape_string(raw_string)?;
        Ok(unescaped)
    }

    pub fn unescape_string(&self, escaped: &str) -> Result<String, RspError> {
        let mut result = String::new();
        let mut chars = escaped.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some(c) => {
                        result.push('\\');
                        result.push(c);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    pub fn serialize_yaml_with_pipes(&self, value: &Value) -> Result<String, RspError> {
        let mut output = String::new();
        self.serialize_value(value, &mut output, 0, &mut std::collections::HashSet::new())?;
        Ok(output)
    }

    fn serialize_value(
        &self,
        value: &Value,
        output: &mut String,
        indent: usize,
        _processed_keys: &mut std::collections::HashSet<String>,
    ) -> Result<(), RspError> {
        match value {
            Value::Mapping(map) => {
                for (key, val) in map {
                    if let Value::String(key_str) = key {
                        self.write_indent(output, indent);
                        output.push_str(&format!("{key_str}:"));

                        if let Value::String(string_val) = val {
                            if self.should_process_key(key_str) && string_val.contains('\n') {
                                output.push_str(" |\n");
                                for line in string_val.lines() {
                                    self.write_indent(output, indent + 1);
                                    output.push_str(line);
                                    output.push('\n');
                                }
                            } else {
                                output.push(' ');
                                self.serialize_value(val, output, 0, _processed_keys)?;
                                output.push('\n');
                            }
                        } else {
                            output.push('\n');
                            self.serialize_value(val, output, indent + 1, _processed_keys)?;
                        }
                    }
                }
            }
            Value::String(s) => {
                if s.contains('\n') || s.contains('"') || s.starts_with(' ') || s.ends_with(' ') {
                    output.push_str(&format!("\"{}\"", s.replace('"', "\\\"")));
                } else {
                    output.push_str(s);
                }
            }
            _ => {
                let serialized = serde_yaml::to_string(value)?;
                output.push_str(serialized.trim());
                if !serialized.ends_with('\n') {
                    output.push('\n');
                }
            }
        }
        Ok(())
    }

    fn write_indent(&self, output: &mut String, indent: usize) {
        output.push_str(&"  ".repeat(indent));
    }
}
