#[derive(Debug)]
pub struct DroppedFile {
    pub path: String,
    pub mime_type: Option<String>,
    pub size: u64,
}

#[derive(Debug)]
pub struct UploadedFile {
    pub file_id: String,
    pub filename: String,
    pub size: u64,
}

#[tokio::test]
async fn drag_and_drop_uploads_file() {
    let file = DroppedFile {
        path: "tests/fixtures/sample.txt".into(),
        mime_type: Some("text/plain".into()),
        size: 128,
    };

    let _uploaded = handle_drop_and_upload(file).await;
}

#[tokio::test]
async fn uploaded_files_appear_in_cache() {
    let _files = list_local_files().await;
}

async fn handle_drop_and_upload(_file: DroppedFile) -> UploadedFile {
    todo!("Implement FileHandlingFlow::handle_drop to satisfy T012 integration test");
}

async fn list_local_files() -> Vec<UploadedFile> {
    todo!("Implement FileHandlingFlow::list_cache to satisfy T012 integration test");
}
