use thiserror::Error;

/// Main error type for the workflow generator
#[derive(Error, Debug)]
pub enum WorkflowError {
    #[error("Analysis failed: {0}")]
    AnalysisError(String),

    #[error("Credential validation failed: {0}")]
    CredentialError(String),

    #[error("Code generation failed: {0}")]
    CodeGenerationError(String),

    #[error("Deployment failed: {0}")]
    DeploymentError(String),

    #[error("AWS error: {0}")]
    AwsError(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid spec: {0}")]
    InvalidSpec(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("UI error: {0}")]
    UiError(String),
}

// Implement From for dialoguer::Error
impl From<dialoguer::Error> for WorkflowError {
    fn from(err: dialoguer::Error) -> Self {
        WorkflowError::UiError(err.to_string())
    }
}

/// Convenience type alias for Results using WorkflowError
pub type Result<T> = std::result::Result<T, WorkflowError>;

impl WorkflowError {
    /// Create an AnalysisError with context
    pub fn analysis<S: Into<String>>(msg: S) -> Self {
        WorkflowError::AnalysisError(msg.into())
    }

    /// Create a CredentialError with context
    pub fn credential<S: Into<String>>(msg: S) -> Self {
        WorkflowError::CredentialError(msg.into())
    }

    /// Create a CodeGenerationError with context
    pub fn code_generation<S: Into<String>>(msg: S) -> Self {
        WorkflowError::CodeGenerationError(msg.into())
    }

    /// Create a DeploymentError with context
    pub fn deployment<S: Into<String>>(msg: S) -> Self {
        WorkflowError::DeploymentError(msg.into())
    }

    /// Create a ConfigError with context
    pub fn config<S: Into<String>>(msg: S) -> Self {
        WorkflowError::ConfigError(msg.into())
    }

    /// Create a ValidationError with context
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        WorkflowError::ValidationError(msg.into())
    }
}
