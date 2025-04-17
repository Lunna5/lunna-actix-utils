use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::{fmt, ops};
use validator::Validate;

/// A wrapper type for automatically validating JSON input in Actix Web handlers.
///
/// `ValidatedJson<T>` works similarly to `web::Json<T>`, but automatically runs
/// validation using the `validator` crate on the deserialized payload. If validation fails,
/// it returns a `400 Bad Request` error.
///
/// # Example
/// ```
/// use actix_web::{post, web, App, HttpServer};
/// use serde::Deserialize;
/// use validator::Validate;
/// use luna_actix_utils::extractors::validated_json::ValidatedJson;
///
/// #[derive(Debug, Deserialize, Validate)]
/// struct MyPayload {
///     #[validate(length(min = 1))]
///     name: String,
/// }
///
/// #[post("/submit")]
/// async fn submit(data: ValidatedJson<MyPayload>) -> String {
///     format!("Hello, {}!", data.name)
/// }
/// ```
pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    /// Consumes the wrapper and returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for ValidatedJson<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Display> fmt::Display for ValidatedJson<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: fmt::Debug> fmt::Debug for ValidatedJson<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: Serialize> Serialize for ValidatedJson<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;

    /// The future that resolves to `ValidatedJson<T>`.
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    /// Extracts and validates a JSON payload from the incoming request.
    ///
    /// This implementation first attempts to extract a `web::Json<T>` from the request,
    /// then validates it using the `Validate` trait. If validation passes, it wraps the
    /// inner value in `ValidatedJson`. Otherwise, it returns a `400 Bad Request`.
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = web::Json::<T>::from_request(req, payload);

        Box::pin(async move {
            let json_wrapper = fut.await?;

            json_wrapper
                .validate()
                .map_err(actix_web::error::ErrorBadRequest)?;

            Ok(ValidatedJson(json_wrapper.into_inner()))
        })
    }
}
