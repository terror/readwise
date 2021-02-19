/// ! Rust wrapper for the Readwise public API. The official readwise public API
/// ! documentation can be found [here](https://readwise.io/api_deets).
/// This wrapper supports retrieving Book information and CRUD for Highlights.
use anyhow::{anyhow, Result};
use http::Method;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const URL: &str = "https://readwise.io";

/// The authenticated client instance. The access_token can be
/// obtained through Readwise.
pub struct Client {
  pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct BooksResponse {
  count:    i64,
  next:     Option<String>,
  previous: Option<String>,
  results:  Vec<Book>,
}

#[derive(Serialize, Deserialize)]
pub struct HighlightsResponse {
  count:    i64,
  next:     Option<String>,
  previous: Option<String>,
  results:  Vec<Highlight>,
}

/// An individual book
#[derive(Serialize, Deserialize, Default)]
pub struct Book {
  pub id:                  i64,
  pub title:               String,
  pub author:              String,
  pub category:            String,
  pub num_highlights:      i64,
  pub last_highlighted_at: Option<String>,
  pub updated:             String,
  pub cover_image_url:     String,
  pub highlights_url:      String,
  pub source_url:          Option<String>,
}

/// An individual highlight
#[derive(Serialize, Deserialize, Default)]
pub struct Highlight {
  pub id:             i64,
  pub text:           String,
  pub note:           String,
  pub location:       i64,
  pub location_type:  String,
  pub highlighted_at: Option<String>,
  pub url:            Option<String>,
  pub color:          String,
  pub updated:        String,
  pub books_id:       Option<String>,
}

fn get_request_url() -> String {
  #[cfg(not(test))]
  let url = format!("{}", URL);
  #[cfg(test)]
  let url = format!("{}", mockito::server_url());
  url
}

impl Client {
  /// Fetch all books from a specified page
  /// Builds and returns a Vec of Book structs
  pub fn books(&self, page: i64) -> Result<Vec<Book>> {
    let mut ret: Vec<Book> = Vec::new();

    let resp = signed_request(
      &format!("/books?page={}", page),
      &self.access_token,
      Method::GET,
      None,
    )?;

    if resp.status().is_success() {
      let response_text = &resp.text()?;
      let data: BooksResponse = serde_json::from_str(&response_text)?;

      for book in data.results {
        ret.push(book);
      }
    } else {
      Err(anyhow!(
        "Failed to fetch books with status code: {}.",
        resp.status()
      ))?
    }
    Ok(ret)
  }

  /// Fetch all highlights from a specified page
  /// Builds and returns a Vec of Highlight structs
  pub fn highlights(&self, page: i64) -> Result<Vec<Highlight>> {
    let mut ret: Vec<Highlight> = Vec::new();

    let resp = signed_request(
      &format!("/highlights?page={}", page),
      &self.access_token,
      Method::GET,
      None,
    )?;

    if resp.status().is_success() {
      let response_text = &resp.text()?;
      let data: HighlightsResponse = serde_json::from_str(&response_text)?;

      for highlight in data.results {
        ret.push(highlight);
      }
    } else {
      Err(anyhow!(
        "Failed to fetch highlights with status code: {}.",
        resp.status()
      ))?
    }
    Ok(ret)
  }

  /// Fetch a single book by ID
  pub fn book(&self, id: i64) -> Result<Book> {
    let resp = signed_request(
      &format!("/books/{}", id),
      &self.access_token,
      Method::GET,
      None,
    )?;
    if resp.status().is_success() {
      let response_text = resp.text()?;
      let data: Book = serde_json::from_str(&response_text)?;
      Ok(data)
    } else {
      Err(anyhow!("Failed to fetch book with id: {}", id))
    }
  }

  /// Fetch a single highlight by ID
  pub fn highlight(&self, id: i64) -> Result<Highlight> {
    let resp = signed_request(
      &format!("/highlights/{}", id),
      &self.access_token,
      Method::GET,
      None,
    )?;
    if resp.status().is_success() {
      let response_text = resp.text()?;
      let data: Highlight = serde_json::from_str(&response_text)?;
      Ok(data)
    } else {
      Err(anyhow!("Failed to fetch highlight with id: {}", id))
    }
  }

  /// Create one or more highlights
  pub fn create(&self, highlights: Vec<Highlight>) -> Result<()> {
    Ok(())
  }

  /// Update a single highlight
  pub fn update(&self, highlight: Highlight) -> Result<()> {
    Ok(())
  }

  /// Delete a single highlight
  pub fn delete(&self, id: i64) -> Result<()> {
    Ok(())
  }
}

fn signed_request(
  endpoint: &str,
  token: &str,
  method: Method,
  body: Option<String>,
) -> Result<reqwest::blocking::Response> {
  let request_client = reqwest::blocking::Client::new();
  let url = format!("{}/api/v2{}", &get_request_url(), endpoint);
  let resp;

  match method {
    Method::GET => {
      resp = request_client
        .get(&url)
        .header(reqwest::header::AUTHORIZATION, format!("Token {}", token))
        .send()?;
    },
    Method::POST => {
      resp = request_client
        .post(&url)
        .header(reqwest::header::AUTHORIZATION, format!("Token {}", token))
        .body(body.unwrap())
        .send()?;
    },
    _ => panic!("Unsupported request method"),
  }

  Ok(resp)
}

/// Authenticate client using a readwise access token
pub fn auth(access_token: &str) -> Result<Client> {
  let resp = signed_request("/auth", access_token, Method::GET, None)?;

  if resp.status().is_success() {
    Ok(Client {
      access_token: access_token.to_string(),
    })
  } else {
    Err(anyhow!(
      "Authentication failed with status code: {}",
      resp.status()
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockito::mock;

  fn client() -> Client {
    Client {
      access_token: String::new(),
    }
  }

  fn get_book_as_string() -> String {
    let book = Book::default();
    serde_json::to_string(&book).unwrap()
  }

  fn get_highlight_as_string() -> String {
    let highlight = Highlight::default();
    serde_json::to_string(&highlight).unwrap()
  }

  #[test]
  fn test_authenticate() {
    let _m = mock("GET", "/api/v2/auth").with_status(204).create();

    let result = auth("token");
    assert!(result.is_ok(), result.err().unwrap().to_string());

    let client = result.unwrap();
    assert_eq!("token", client.access_token);
  }

  #[test]
  fn test_authenticate_bad_token() {
    let _m = mock("GET", "/api/v2/auth").with_status(401).create();

    let result = auth("token");
    assert!(result.is_err(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_books() {
    let _m = mock("GET", "/api/v2/books?page=1")
      .with_status(200)
      .with_body(format!(
        r#" {{ "count": 1, "next": null, "previous": null, "results": [{}] }} "#,
        &get_book_as_string()
      ))
      .create();

    let result = client().books(1);
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_highlights() {
    let _m = mock("GET", "/api/v2/highlights?page=1")
      .with_status(200)
      .with_body(format!(
        r#" {{ "count": 1, "next": null, "previous": null, "results": [{}] }} "#,
        &get_highlight_as_string()
      ))
      .create();

    let result = client().highlights(1);
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_single_book() {
    let _m = mock("GET", "/api/v2/books/1")
      .with_status(200)
      .with_body(format!("{}", &get_book_as_string()))
      .create();

    let result = client().book(1);
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_single_highlight() {
    let _m = mock("GET", "/api/v2/highlights/1")
      .with_status(200)
      .with_body(format!("{}", &get_highlight_as_string()))
      .create();

    let result = client().highlight(1);
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }
}
