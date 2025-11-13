/// Code packager for workflow deployment
use crate::compiler::code_generator::GeneratedCode;
use crate::utils::errors::{Result, WorkflowError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Package manifest metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub workflow_id: Uuid,
    pub workflow_name: String,
    pub version: String,
    pub created_at: String,
    pub files: Vec<PackageFile>,
    pub checksum: String,
}

/// File metadata in package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageFile {
    pub name: String,
    pub size: u64,
    pub checksum: String,
}

/// Packaged workflow artifact
#[derive(Debug, Clone)]
pub struct PackageArtifact {
    pub manifest: PackageManifest,
    pub package_path: PathBuf,
    pub size_bytes: u64,
}

/// Code packager for creating deployment artifacts
pub struct CodePackager {
    output_dir: PathBuf,
}

impl CodePackager {
    /// Create a new code packager
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    /// Package workflow code for deployment
    pub async fn package(
        &self,
        workflow_id: Uuid,
        workflow_name: &str,
        generated_code: &GeneratedCode,
    ) -> Result<PackageArtifact> {
        tracing::info!("Packaging workflow {} for deployment", workflow_id);

        // Create package directory
        let package_dir = self.output_dir.join(workflow_id.to_string());
        fs::create_dir_all(&package_dir).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to create package directory: {}", e))
        })?;

        // Write workflow files
        let mut files = Vec::new();

        // Write workflow.py
        let workflow_path = package_dir.join("workflow.py");
        fs::write(&workflow_path, &generated_code.workflow_code).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to write workflow.py: {}", e))
        })?;
        files.push(self.create_file_metadata(&workflow_path)?);

        // Write worker.py
        let worker_path = package_dir.join("worker.py");
        fs::write(&worker_path, &generated_code.worker_code).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to write worker.py: {}", e))
        })?;
        files.push(self.create_file_metadata(&worker_path)?);

        // Write requirements.txt
        let requirements_path = package_dir.join("requirements.txt");
        fs::write(&requirements_path, &generated_code.requirements).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to write requirements.txt: {}", e))
        })?;
        files.push(self.create_file_metadata(&requirements_path)?);

        // Write Dockerfile
        let dockerfile_path = package_dir.join("Dockerfile");
        fs::write(&dockerfile_path, &generated_code.dockerfile).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to write Dockerfile: {}", e))
        })?;
        files.push(self.create_file_metadata(&dockerfile_path)?);

        // Calculate package checksum
        let package_checksum = self.calculate_package_checksum(&files)?;

        // Create manifest
        let manifest = PackageManifest {
            workflow_id,
            workflow_name: workflow_name.to_string(),
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            files: files.clone(),
            checksum: package_checksum,
        };

        // Write manifest
        let manifest_path = package_dir.join("manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to serialize manifest: {}", e))
        })?;
        fs::write(&manifest_path, manifest_json).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to write manifest: {}", e))
        })?;

        // Calculate total package size
        let size_bytes = self.calculate_directory_size(&package_dir)?;

        tracing::info!(
            "Packaged workflow {} ({} bytes) at {:?}",
            workflow_id,
            size_bytes,
            package_dir
        );

        Ok(PackageArtifact {
            manifest,
            package_path: package_dir,
            size_bytes,
        })
    }

    /// Create file metadata with checksum
    fn create_file_metadata(&self, path: &Path) -> Result<PackageFile> {
        let metadata = fs::metadata(path).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to read file metadata: {}", e))
        })?;

        let content = fs::read(path)
            .map_err(|e| WorkflowError::DeploymentError(format!("Failed to read file: {}", e)))?;

        let checksum = self.calculate_checksum(&content);

        Ok(PackageFile {
            name: path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            size: metadata.len(),
            checksum,
        })
    }

    /// Calculate SHA-256 checksum
    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Calculate package checksum from all files
    fn calculate_package_checksum(&self, files: &[PackageFile]) -> Result<String> {
        let mut combined = String::new();
        for file in files {
            combined.push_str(&file.checksum);
        }
        Ok(self.calculate_checksum(combined.as_bytes()))
    }

    /// Calculate total directory size
    fn calculate_directory_size(&self, dir: &Path) -> Result<u64> {
        let mut total_size = 0u64;

        for entry in fs::read_dir(dir).map_err(|e| {
            WorkflowError::DeploymentError(format!("Failed to read directory: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                WorkflowError::DeploymentError(format!("Failed to read directory entry: {}", e))
            })?;

            let metadata = entry.metadata().map_err(|e| {
                WorkflowError::DeploymentError(format!("Failed to read entry metadata: {}", e))
            })?;

            if metadata.is_file() {
                total_size += metadata.len();
            }
        }

        Ok(total_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_package_manifest_serialization() {
        let manifest = PackageManifest {
            workflow_id: Uuid::new_v4(),
            workflow_name: "test_workflow".to_string(),
            version: "1.0.0".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            files: vec![],
            checksum: "abc123".to_string(),
        };

        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: PackageManifest = serde_json::from_str(&json).unwrap();

        assert_eq!(manifest.workflow_id, deserialized.workflow_id);
        assert_eq!(manifest.workflow_name, deserialized.workflow_name);
    }

    #[test]
    fn test_code_packager_creation() {
        let temp_dir = tempdir().unwrap();
        let packager = CodePackager::new(temp_dir.path().to_path_buf());

        assert!(packager.output_dir.exists());
    }
}
