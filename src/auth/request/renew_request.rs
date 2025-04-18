use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents a token renewal request.
///
/// Typically used to request a new access token using a refresh token or similar mechanism.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RenewRequest {
    /// The refresh token or renewal token.
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
}

/// Trait that defines the expected behavior of any type representing a token renewal request.
///
/// Allows for flexibility in handling different input types while following the same interface.
pub trait RenewRequestLike {
    /// Returns the renewal token.
    fn token(&self) -> &str;
}

/// Implements `RenewRequestLike` for `RenewRequest`,
/// so it can be used where the trait is expected.
impl RenewRequestLike for RenewRequest {
    fn token(&self) -> &str {
        &self.token
    }
}
