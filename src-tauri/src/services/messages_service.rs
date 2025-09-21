use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::auth_service::SuccessResponse;
use super::{ApiError, LibreChatClient};

#[derive(Clone)]
pub struct MessagesService {
    client: LibreChatClient,
}

impl MessagesService {
    pub fn new(client: LibreChatClient) -> Self {
        Self { client }
    }

    pub async fn send(&self, request: &SendMessageRequest) -> Result<MessageResponse, ApiError> {
        self.client
            .post_json::<_, MessageResponse>("/messages", request)
            .await
    }

    pub async fn get(&self, message_id: &str) -> Result<Message, ApiError> {
        let path = format!("/messages/{}", message_id);
        self.client.get_json(&path).await
    }

    pub async fn update(
        &self,
        message_id: &str,
        request: &UpdateMessageRequest,
    ) -> Result<Message, ApiError> {
        let path = format!("/messages/{}", message_id);
        self.client.put_json(&path, request).await
    }

    pub async fn delete(&self, message_id: &str) -> Result<SuccessResponse, ApiError> {
        let path = format!("/messages/{}", message_id);
        self.client.delete_json(&path).await
    }

    pub async fn abort(&self, request: &AbortMessageRequest) -> Result<SuccessResponse, ApiError> {
        self.client
            .post_json::<_, SuccessResponse>("/messages/abort", request)
            .await
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageRequest {
    pub text: String,
    pub conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMessageRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbortMessageRequest {
    pub conversation_id: String,
    pub message_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub message: Message,
    #[serde(default)]
    pub conversation: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message_id: String,
    pub conversation_id: String,
    pub text: String,
    pub sender: String,
    pub created_at: String,
    #[serde(default)]
    pub is_created_by_user: bool,
    #[serde(default)]
    pub error: bool,
    #[serde(default)]
    pub parent_message_id: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub token_count: Option<u32>,
    #[serde(default)]
    pub finish_reason: Option<String>,
    #[serde(default)]
    pub files: Vec<MessageFile>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageFile {
    pub file_id: String,
    pub filename: String,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
}
