use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

/// Represents a user registration request.
///
/// This struct is used to capture user input when creating a new account.
/// It includes validation for all fields.
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    /// The desired username.
    ///
    /// Must be between 4 and 30 characters.
    #[validate(length(min = 4, max = 30))]
    #[schema(example = "new_user")]
    pub username: String,

    /// The user's email address.
    ///
    /// Must be a valid email format.
    #[validate(email)]
    #[schema(example = "user@example.com")]
    pub email: String,

    /// The password for the new account.
    ///
    /// Must be between 4 and 30 characters.
    #[validate(length(min = 4, max = 30))]
    #[schema(example = "securePass123")]
    pub password: String,
}

/// Trait that defines the expected behavior of any type representing a registration request.
///
/// This allows using different implementations of registration input as long as they
/// conform to this interface.
pub trait RegisterRequestLike {
    /// Returns the username.
    fn username(&self) -> &str;

    /// Returns the email address.
    fn email(&self) -> &str;

    /// Returns the password.
    fn password(&self) -> &str;
}

/// Implements `RegisterRequestLike` for `RegisterRequest`,
/// allowing it to be used wherever the trait is expected.
impl RegisterRequestLike for RegisterRequest {
    fn username(&self) -> &str {
        &self.username
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn password(&self) -> &str {
        &self.password
    }
}
