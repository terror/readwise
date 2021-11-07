// std
pub use std::collections::HashMap;

// dependencies
pub use {
  http::Method,
  reqwest::{
    blocking::{self, Response},
    header, StatusCode,
  },
  serde::{Deserialize, Serialize},
  serde_json,
  snafu::{ResultExt, Snafu},
};

// modules
pub(crate) use crate::error;

// functions
pub use crate::{auth::auth, request::signed_request, url::request_url};

// structs and enums
pub use crate::{
  client::Client,
  error::Error,
  model::{Book, BooksResponse, Highlight, HighlightCreateResponse, HighlightsResponse},
};

// type aliases
pub type Result<T, E = Error> = std::result::Result<T, E>;

// test dependencies
#[cfg(test)]
pub use mockito;
