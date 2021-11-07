use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(context(false), display("Request error: {}", source))]
  Client { source: reqwest::Error },

  #[snafu(context(false), display("Serde JSON error: {}", source))]
  Deserialize { source: serde_json::Error },

  #[snafu(context(false), display("Invalid header value: {}", source))]
  HeaderValue { source: header::InvalidHeaderValue },

  #[snafu(display("Unsupported request method: {}", method.to_string()))]
  UnsupportedRequest { method: Method },

  #[snafu(display("Bad request: {}", status.to_string()))]
  BadRequest { status: StatusCode },
}
