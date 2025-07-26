use thiserror::Error;

#[derive(Error, Debug)]
pub enum RspError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Processing error: {0}")]
    Processing(String),
}