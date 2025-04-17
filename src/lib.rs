//! `lunna_actix_utils` is a utility crate by [lunna.dev (me)](https://lunna.dev) for Actix Web projects.
//!
//! This crate contains reusable tools and extractors that are used across several of my
//! personal projects, aiming to reduce boilerplate and keep code clean and maintainable.
//!
//! It is not intended as a general-purpose library for public use, but you're welcome
//! to explore or adapt anything you find useful.
//!
//! # Features
//!
//! - Custom extractors (e.g. [`extractors::validated_json::ValidatedJson`] for automatic validation of JSON payloads)
//! - Small utilities to make working with Actix Web smoother
//!
//! # Modules
//!
//! - [`extractors`] — Request extractors with extra behavior like input validation.

pub mod extractors;
