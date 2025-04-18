use async_trait::async_trait;

use crate::auth::{
    error::AuthError,
    request::{
        login_request::LoginRequestLike, register_request::RegisterRequestLike,
        renew_request::RenewRequestLike,
    },
    response::token_response::TokenResponse,
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, login_request: &dyn LoginRequestLike)
    -> Result<TokenResponse, AuthError>;

    async fn register(
        &self,
        register_request: &dyn RegisterRequestLike,
    ) -> Result<TokenResponse, AuthError>;

    async fn renew(&self, renew_request: &dyn RenewRequestLike)
    -> Result<TokenResponse, AuthError>;
}
