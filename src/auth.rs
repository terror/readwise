use crate::common::*;

/// Authenticate client using a readwise access token
pub fn auth(access_token: &str) -> Result<Client> {
  signed_request("/auth", access_token, Method::GET, None)?;

  Ok(Client {
    access_token: access_token.to_string(),
  })
}
