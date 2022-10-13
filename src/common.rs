pub(crate) use std::collections::HashMap;

pub(crate) use {
  http::Method,
  reqwest::{
    blocking::{self, Response},
    header, StatusCode,
  },
  serde::{Deserialize, Serialize},
  snafu::Snafu,
};

pub(crate) use crate::{error, url::request_url};

pub(crate) use crate::{
  error::Error,
  model::{
    Book, BooksResponse, Highlight, HighlightCreateResponse, HighlightsResponse,
  },
};

pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;
