//! This module contains custom extractors used in Actix Web handlers.
//!
//! Extractors are responsible for extracting and transforming parts of an incoming HTTP
//! request into handler arguments. These custom extractors extend the built-in capabilities
//! of Actix Web to add extra functionality like input validation.
//!
//! # Modules
//!
//! - [`validated_json`] — A JSON extractor that automatically validates input using the
//!   [`validator`](https://docs.rs/validator) crate.
pub mod validated_json;