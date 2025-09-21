use std::path::{Path, PathBuf};

use mime::Mime;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;

use super::auth_service::SuccessResponse;
use super::{ApiError, LibreChatClient};

#[derive(Clone)]
pub struct FilesService {
    client: LibreChatClient,
}

impl FilesService {
    pub fn new(client: LibreChatClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: &ListFilesQuery) -> Result<FileListResponse, ApiError> {
        self.client
            .get_json_with_query("/files", params)
            .await
    }

    pub async fn upload(&self, request: &FileUploadRequest) -> Result<FileUploadResponse, ApiError> {
        let bytes = fs::read(&request.file_path).await?;
        let file_name = request
            .file_name
            .clone()
            .unwrap_or_else(|| default_file_name(&request.file_path));

        let part = build_file_part(&bytes, &file_name, request.mime_type.as_deref())?;

        let mut form = Form::new().part("file", part);
        if let Some(ref usage) = request.usage {
            form = form.text("usage", usage.clone());
        }
        if let Some(ref context) = request.context {
            form = form.text("context", context.clone());
        }
        if let Some(ref assistant_id) = request.assistant_id {
            form = form.text("assistantId", assistant_id.clone());
        }
        if let Some(ref agent_id) = request.agent_id {
            form = form.text("agentId", agent_id.clone());
        }
        if let Some(ref conversation_id) = request.conversation_id {
            form = form.text("conversationId", conversation_id.clone());
        }
        if let Some(ref message_id) = request.message_id {
            form = form.text("messageId", message_id.clone());
        }

        self.client.post_multipart("/files", form).await
    }

    pub async fn get(&self, file_id: &str) -> Result<FileDetail, ApiError> {
        let path = format!("/files/{}", file_id);
        self.client.get_json(&path).await
    }

    pub async fn delete(&self, file_id: &str) -> Result<SuccessResponse, ApiError> {
        let path = format!("/files/{}", file_id);
        self.client.delete_json(&path).await
    }

    pub async fn batch_delete(&self, request: &BatchDeleteRequest) -> Result<BatchDeleteResponse, ApiError> {
        self.client
            .delete_with_json("/files/batch/delete", request)
            .await
    }
}

fn build_file_part(bytes: &[u8], file_name: &str, mime: Option<&str>) -> Result<Part, ApiError> {
    let mut part = Part::bytes(bytes.to_owned()).file_name(file_name.to_string());
    if let Some(mime_str) = mime {
        let parsed: Mime = mime_str.parse().map_err(|_| {
            ApiError::Http {
                status: reqwest::StatusCode::BAD_REQUEST,
                message: format!("invalid mime type: {}", mime_str),
            }
        })?;
        part = part.mime_str(parsed.as_ref())?;
    }
    Ok(part)
}

fn default_file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("upload.bin")
        .to_string()
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFilesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadRequest {
    pub file_path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileListResponse {
    pub files: Vec<FileDetail>,
    pub total: u32,
    #[serde(default)]
    pub page_size: Option<u32>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub has_more: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub file_id: String,
    pub filename: String,
    #[serde(default)]
    pub originalname: Option<String>,
    #[serde(rename = "type")]
    pub mime_type: String,
    #[serde(default)]
    pub usage: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub filepath: Option<String>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub context: Option<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
    #[serde(rename = "bytes")]
    pub size_bytes: u64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadResponse {
    pub files: Vec<FileDetail>,
    pub message: String,
    #[serde(default)]
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteRequest {
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteResponse {
    pub deleted: Vec<String>,
    pub errors: Vec<Value>,
    #[serde(default)]
    pub message: Option<String>,
}
