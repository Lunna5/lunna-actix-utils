use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use strum::{AsRefStr, EnumString};
use thiserror::Error;
use crate::util::text_util::TextUtil;

#[derive(Debug, Error, AsRefStr, EnumString)]
pub enum AuthError {
  #[error("Invalid username or password")]
  InvalidUsernameOrPassword,
  #[error("Invalid email")]
  InvalidEmail,
  #[error("Invalid password, check the password requirements")]
  InvalidPassword,
  #[error("The email is already taken")]
  EmailAlreadyInUse,
  #[error("The email already taken")]
  UsernameAlreadyInUse,
  #[error("Invalid captcha")]
  InvalidCaptcha,
  #[error("Invalid token")]
  InvalidToken,
  #[error("Token expired")]
  TokenExpired,
  #[error("Token not found")]
  TokenNotFound,
  #[error("Token not valid")]
  TokenNotValid,
  #[error("No private key was provided")]
  NoPrivateKey,
  #[error("Internal error during authentication")]
  InternalError,
}

impl AuthError {
  pub fn i18n_key(&self) -> String {
    TextUtil::i18n_key_with_prefix("auth", self)
  }
}

impl Serialize for AuthError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer
  {
    let error_message = self.to_string();
    let i18n_key = self.i18n_key();

    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("error", &error_message)?;
    map.serialize_entry("key", &i18n_key)?;
    map.end()
  }
}