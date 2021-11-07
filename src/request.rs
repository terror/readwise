use crate::common::*;

pub fn signed_request(
  endpoint: &str,
  token: &str,
  method: Method,
  body: Option<HashMap<&str, Vec<HashMap<&str, &str>>>>,
) -> Result<Response> {
  let url = format!("{}/api/v2{}", &request_url(), endpoint);

  let mut headers = header::HeaderMap::new();

  headers.insert(
    header::AUTHORIZATION,
    header::HeaderValue::from_str(&format!("Token {}", token))?,
  );

  let request_client = blocking::Client::builder()
    .default_headers(headers)
    .build()?;

  let request = match method {
    Method::GET => Ok(request_client.get(&url)),
    Method::POST => Ok(request_client.post(&url).json(&body.unwrap())),
    Method::PATCH => Ok(request_client.patch(&url).json(&body.unwrap()["body"][0])),
    Method::DELETE => Ok(request_client.delete(&url)),
    _ => Err(error::Error::UnsupportedRequest { method }),
  };

  let response = request?.send()?;

  match response.status().is_success() {
    true => Ok(response),
    false => Err(error::Error::BadRequest {
      status: response.status(),
    }),
  }
}
