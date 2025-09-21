use serde::{Deserialize, Serialize};

use super::auth_service::SuccessResponse;
use super::messages_service::Message as RemoteMessage;
use super::{ApiError, LibreChatClient};

#[derive(Clone)]
pub struct ConversationsService {
    client: LibreChatClient,
}

impl ConversationsService {
    pub fn new(client: LibreChatClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: &ListConversationsParams) -> Result<ConversationListResponse, ApiError> {
        self.client
            .get_json_with_query("/convos", params)
            .await
    }

    pub async fn create(&self, request: &CreateConversationRequest) -> Result<ConversationDetail, ApiError> {
        self.client
            .post_json::<_, ConversationDetail>("/convos", request)
            .await
    }

    pub async fn get(&self, conversation_id: &str) -> Result<ConversationDetail, ApiError> {
        let path = format!("/convos/{}", conversation_id);
        self.client.get_json(&path).await
    }

    pub async fn update(
        &self,
        conversation_id: &str,
        request: &UpdateConversationRequest,
    ) -> Result<ConversationDetail, ApiError> {
        let path = format!("/convos/{}", conversation_id);
        self.client.put_json(&path, request).await
    }

    pub async fn delete(&self, conversation_id: &str) -> Result<SuccessResponse, ApiError> {
        let path = format!("/convos/{}", conversation_id);
        self.client.delete_json(&path).await
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListConversationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationListResponse {
    pub conversations: Vec<ConversationSummary>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationSummary {
    pub conversation_id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_pinned: bool,
    pub is_starred: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationDetail {
    #[serde(flatten)]
    pub summary: ConversationSummary,
    #[serde(default)]
    pub messages: Vec<RemoteMessage>,
}
