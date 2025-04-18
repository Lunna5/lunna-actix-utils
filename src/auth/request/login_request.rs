use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Represents a standard login request.
///
/// This struct can be used directly with frameworks like Actix-Web or Axum
/// to deserialize JSON, form-data, or other types of login payloads.
///
/// Includes basic validation for username and password fields.
#[derive(Debug, Serialize, Deserialize, Validate, Clone, ToSchema)]
pub struct LoginRequest {
    /// The username, email or user identifier.
    ///
    /// Must be between 4 and 256 characters.
    /// This limit is to be inside the limit of 254 characters of an email
    #[validate(length(min = 4, max = 256))]
    #[schema(example = "user@lunna.dev")]
    pub username: String,

    /// The user's password.
    ///
    /// Must be between 8 and 128 characters.
    #[validate(length(min = 8, max = 128))]
    #[schema(example = "password1234")]
    pub password: String,

    /// Indicates whether the session should be persistent (i.e., "remember me").
    #[schema(example = true)]
    pub remember_me: bool,
}

/// A trait that defines the expected behavior of any type representing a login request.
///
/// This allows you to use custom types for login, as long as they implement this trait.
pub trait LoginRequestLike {
    /// Returns the username.
    fn username(&self) -> &str;

    /// Returns the password.
    fn password(&self) -> &str;

    /// Returns the "remember me" flag.
    fn remember_me(&self) -> bool;
}

/// Implements `LoginRequestLike` for `LoginRequest`,
/// allowing it to be used wherever the trait is expected.
impl LoginRequestLike for LoginRequest {
    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn remember_me(&self) -> bool {
        self.remember_me
    }
}
