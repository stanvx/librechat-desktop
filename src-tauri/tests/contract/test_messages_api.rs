use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    #[serde(rename = "parentMessageId")]
    pub parent_message_id: Option<String>,
    pub model: Option<String>,
    pub endpoint: Option<String>,
    pub files: Option<Vec<String>>,
    #[serde(rename = "presetId")]
    pub preset_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateMessageRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct AbortMessageRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
}

#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    pub message: serde_json::Value,
    #[allow(dead_code)]
    pub conversation: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[allow(dead_code)]
    pub text: String,
    #[allow(dead_code)]
    pub sender: String,
    #[allow(dead_code)]
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[tokio::test]
async fn send_message_streams_response() {
    let request = SendMessageRequest {
        text: "Hello, LibreChat".into(),
        conversation_id: "conv_123".into(),
        parent_message_id: None,
        model: Some("gpt-4".into()),
        endpoint: Some("openAI".into()),
        files: None,
        preset_id: None,
    };

    let _response = send_message(request).await;
}

#[tokio::test]
async fn get_message_returns_full_payload() {
    let message_id = "msg_123".to_string();
    let _message = get_message(message_id).await;
}

#[tokio::test]
async fn update_message_edits_text() {
    let message_id = "msg_123".to_string();
    let request = UpdateMessageRequest {
        text: "Updated response".into(),
    };

    let _message = update_message(message_id, request).await;
}

#[tokio::test]
async fn delete_message_acknowledges_success() {
    let message_id = "msg_123".to_string();
    let _response = delete_message(message_id).await;
}

#[tokio::test]
async fn abort_message_cancels_stream() {
    let request = AbortMessageRequest {
        conversation_id: "conv_123".into(),
        message_id: "msg_123".into(),
    };

    let _response = abort_message(request).await;
}

async fn send_message(_request: SendMessageRequest) -> MessageResponse {
    todo!("Implement MessagesService::send to satisfy T008 contract test");
}

async fn get_message(_message_id: String) -> Message {
    todo!("Implement MessagesService::get to satisfy T008 contract test");
}

async fn update_message(_message_id: String, _request: UpdateMessageRequest) -> Message {
    todo!("Implement MessagesService::update to satisfy T008 contract test");
}

async fn delete_message(_message_id: String) -> SuccessResponse {
    todo!("Implement MessagesService::delete to satisfy T008 contract test");
}

async fn abort_message(_request: AbortMessageRequest) -> SuccessResponse {
    todo!("Implement MessagesService::abort to satisfy T008 contract test");
}
