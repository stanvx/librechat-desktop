use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{ApiError, LibreChatClient};

#[derive(Clone)]
pub struct AuthService {
    client: LibreChatClient,
}

impl AuthService {
    pub fn new(client: LibreChatClient) -> Self {
        Self { client }
    }

    pub async fn login_with_password(
        &self,
        request: &PasswordLoginRequest,
    ) -> Result<AuthResponse, ApiError> {
        let response = self
            .client
            .post_json::<_, AuthResponse>("/auth/login", request)
            .await?;
        self.client.set_auth_token(response.token.clone());
        Ok(response)
    }

    pub async fn login_with_oauth(
        &self,
        request: &OAuthLoginRequest,
    ) -> Result<AuthResponse, ApiError> {
        let response = self
            .client
            .post_json::<_, AuthResponse>("/auth/login", request)
            .await?;
        self.client.set_auth_token(response.token.clone());
        Ok(response)
    }

    pub async fn refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> Result<AuthResponse, ApiError> {
        let response = self
            .client
            .post_json::<_, AuthResponse>("/auth/refresh", request)
            .await?;
        self.client.set_auth_token(response.token.clone());
        Ok(response)
    }

    pub async fn logout(&self) -> Result<SuccessResponse, ApiError> {
        let response = self
            .client
            .post_json::<_, SuccessResponse>("/auth/logout", &json!({}))
            .await?;
        self.client.clear_auth_token();
        Ok(response)
    }

    pub async fn current_user(&self) -> Result<UserInfo, ApiError> {
        self.client.get_json("/auth/user").await
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordLoginRequest {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthLoginRequest {
    pub provider: String,
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    #[serde(rename = "expiresIn")]
    pub expires_in: Option<u64>,
    pub user: UserInfo,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub role: Option<String>,
    #[serde(default)]
    pub preferences: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}
