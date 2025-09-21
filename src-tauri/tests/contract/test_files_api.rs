use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ListFilesQuery {
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    pub cursor: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FileUploadRequest {
    pub usage: Option<String>,
    pub context: Option<String>,
    #[serde(rename = "assistantId")]
    pub assistant_id: Option<String>,
    #[serde(rename = "agentId")]
    pub agent_id: Option<String>,
    #[serde(rename = "conversationId")]
    pub conversation_id: Option<String>,
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(skip_serializing)]
    pub file_path: String,
}

#[derive(Debug, Serialize)]
pub struct BatchDeleteRequest {
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileDetail {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[allow(dead_code)]
    pub filename: String,
    #[allow(dead_code)]
    pub size: u64,
    #[allow(dead_code)]
    pub type_field: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct FileListResponse {
    pub files: Vec<FileDetail>,
    pub total: u32,
    #[allow(dead_code)]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    #[allow(dead_code)]
    pub cursor: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "hasMore")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FileUploadResponse {
    pub files: Vec<FileDetail>,
    pub message: String,
    #[allow(dead_code)]
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct BatchDeleteResponse {
    pub deleted: Vec<String>,
    pub errors: Vec<serde_json::Value>,
    #[allow(dead_code)]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

#[tokio::test]
async fn list_files_respects_pagination() {
    let query = ListFilesQuery {
        page_size: Some(25),
        cursor: None,
        source: Some("chat".into()),
    };

    let _response = list_files(query).await;
}

#[tokio::test]
async fn upload_file_returns_metadata() {
    let request = FileUploadRequest {
        usage: Some("chat".into()),
        context: Some("chat".into()),
        assistant_id: None,
        agent_id: None,
        conversation_id: Some("conv_123".into()),
        message_id: None,
        file_path: "tests/fixtures/sample.txt".into(),
    };

    let _upload_response = upload_file(request).await;
}

#[tokio::test]
async fn get_file_returns_detail() {
    let file_id = "file_123".to_string();
    let _detail = get_file(file_id).await;
}

#[tokio::test]
async fn delete_file_returns_success() {
    let file_id = "file_123".to_string();
    let _response = delete_file(file_id).await;
}

#[tokio::test]
async fn batch_delete_returns_deleted_ids() {
    let request = BatchDeleteRequest {
        files: vec!["file_123".into(), "file_456".into()],
    };

    let _response = batch_delete(request).await;
}

async fn list_files(_query: ListFilesQuery) -> FileListResponse {
    todo!("Implement FilesService::list to satisfy T009 contract test");
}

async fn upload_file(_request: FileUploadRequest) -> FileUploadResponse {
    todo!("Implement FilesService::upload to satisfy T009 contract test");
}

async fn get_file(_file_id: String) -> FileDetail {
    todo!("Implement FilesService::get to satisfy T009 contract test");
}

async fn delete_file(_file_id: String) -> SuccessResponse {
    todo!("Implement FilesService::delete to satisfy T009 contract test");
}

async fn batch_delete(_request: BatchDeleteRequest) -> BatchDeleteResponse {
    todo!("Implement FilesService::batch_delete to satisfy T009 contract test");
}
