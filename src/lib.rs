//! Rust wrapper for the Readwise public API. The official readwise public API
//! documentation can be found [here](https://readwise.io/api_deets).
//! This wrapper supports retrieving Book information and CRUD for Highlights.
//!
//! ## Installation
//! Simply add `readwise = "0.1.0"` to your Cargo.toml
//!
//! ## Example
//! ```no_run
//! use readwise::auth;
//!
//! extern crate dotenv;
//!
//! use dotenv::dotenv;
//! use std::{collections::HashMap, env};
//!
//! fn main() -> Result<(), anyhow::Error> {
//!   dotenv().ok();
//!
//!   let client = auth(&env::var("ACCESS_TOKEN").unwrap()).unwrap();
//!
//!   // Fetch all books on page 1
//!   for book in client.books(1).unwrap() {
//!     println!("{}", book.title);
//!   }
//!
//!   // Fetch all highlights on page 1
//!   for highlight in client.highlights(1).unwrap() {
//!     println!("{}", highlight.id);
//!   }
//!
//!   // Create highlight(s)
//!   let mut highlights = Vec::new();
//!   let mut highlight = HashMap::new();
//!
//!   highlight.insert("text", "hello world!");
//!   highlights.push(highlight);
//!
//!   let result = client.create(highlights)?;
//!
//!   for highlight in result {
//!     println!("{}", highlight.text);
//!   }
//!
//!   // Update a highlight by ID
//!   let mut fields = HashMap::new();
//!   fields.insert("text", "hello, world!");
//!
//!   let _result = client.update(138105649, fields)?;
//!
//!   // Delete a highlight by ID
//!   client.delete(136887156)?;
//!
//!   Ok(())
//! }
//! ```
use anyhow::{anyhow, Result};
use http::Method;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

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
struct BooksResponse {
  count:    i64,
  next:     Option<String>,
  previous: Option<String>,
  results:  Vec<Book>,
}

#[derive(Serialize, Deserialize)]
struct HighlightsResponse {
  count:    i64,
  next:     Option<String>,
  previous: Option<String>,
  results:  Vec<Highlight>,
}

/// A reponse from creating new highlights
#[derive(Serialize, Deserialize, Default)]
struct HighlightCreateResponse {
  id:                  i64,
  title:               String,
  auhtor:              Option<String>,
  category:            String,
  num_highlights:      i64,
  last_highlighted_at: Option<String>,
  updated:             String,
  cover_image_url:     String,
  highlights_url:      String,
  source_url:          Option<String>,
  modified_highlights: Vec<i64>,
}

/// An individual book
#[derive(Serialize, Deserialize, Default)]
pub struct Book {
  pub id:                  i64,
  pub title:               String,
  pub author:              Option<String>,
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

  /// Create one or more highlights and return them
  pub fn create(&self, highlights: Vec<HashMap<&str, &str>>) -> Result<Vec<Highlight>> {
    let mut body = HashMap::new();
    body.insert("highlights", highlights);

    let resp = signed_request(
      &format!("/highlights"),
      &self.access_token,
      Method::POST,
      Some(body),
    )?;

    if resp.status().is_success() {
      let response_text = resp.text()?;

      let highlight_information: Vec<HighlightCreateResponse> =
        serde_json::from_str(&response_text)?;

      // build the return vec
      let mut created_highlights: Vec<Highlight> = Vec::new();

      for response_item in highlight_information {
        // fetch individual highlights
        for id in response_item.modified_highlights {
          let highlight = self.highlight(id)?;
          created_highlights.push(highlight);
        }
      }

      Ok(created_highlights)
    } else {
      Err(anyhow!(
        "Failed to create new highlights: {}",
        resp.status()
      ))
    }
  }

  /// Update a single highlight and return it
  pub fn update(&self, id: i64, body: HashMap<&str, &str>) -> Result<Highlight> {
    let mut container = HashMap::new();

    let mut highlights = Vec::new();

    highlights.push(body);

    container.insert("body", highlights);

    let resp = signed_request(
      &format!("/highlights/{}", id),
      &self.access_token,
      Method::PATCH,
      Some(container),
    )?;

    if resp.status().is_success() {
      let response_text = &resp.text()?;
      let updated_highlight: Highlight = serde_json::from_str(&response_text)?;
      Ok(updated_highlight)
    } else {
      Err(anyhow!("Failed to update highlight with id: {}", id))
    }
  }

  /// Delete a single highlight
  pub fn delete(&self, id: i64) -> Result<()> {
    let resp = signed_request(
      &format!("/highlights/{}", id),
      &self.access_token,
      Method::DELETE,
      None,
    )?;

    if resp.status().is_success() {
      Ok(())
    } else {
      Err(anyhow!("Failed to delete highlight with id: {}", id))
    }
  }
}

fn signed_request(
  endpoint: &str,
  token: &str,
  method: Method,
  body: Option<HashMap<&str, Vec<HashMap<&str, &str>>>>,
) -> Result<reqwest::blocking::Response> {
  let url = format!("{}/api/v2{}", &get_request_url(), endpoint);

  let mut headers = header::HeaderMap::new();
  headers.insert(
    header::AUTHORIZATION,
    header::HeaderValue::from_str(&format!("Token {}", token)).unwrap(),
  );

  let request_client = reqwest::blocking::Client::builder()
    .default_headers(headers)
    .build()?;

  let resp;

  match method {
    Method::GET => {
      resp = request_client.get(&url);
    },
    Method::POST => {
      resp = request_client.post(&url).json(&body.unwrap());
    },
    Method::PATCH => {
      resp = request_client.patch(&url).json(&body.unwrap()["body"][0]);
    },
    Method::DELETE => {
      resp = request_client.delete(&url);
    },
    _ => panic!("Unsupported request method"),
  }

  Ok(resp.send()?)
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

  #[test]
  fn test_create_highlights() {
    let _m = mock("POST", "/api/v2/highlights")
      .with_status(200)
      .with_body(
        r#"
        [ { "id": 1,
          "title": "Quotes",
          "author": null,
          "category": "books",
          "num_highlights": 5,
          "last_highlight_at": "2021-02-20T16:28:53.900414Z",
          "updated": "2021-02-20T16:35:41.793746Z",
          "cover_image_url": "https://readwise-assets.s3.amazonaws.com/static/images/default-book-icon-7.09749d3efd49.png",
          "highlights_url": "https://readwise.io/bookreview/7843339",
          "source_url": null,
          "modified_highlights": [] }
        ]"#,
      )
      .create();

    let result = client().create(Vec::new());
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_update_highlight() {
    let _m = mock("PATCH", "/api/v2/highlights/0")
      .with_status(200)
      .with_body(format!("{}", &get_highlight_as_string()))
      .create();

    let result = client().update(0, HashMap::new());
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_delete_highlight() {
    let _m = mock("DELETE", "/api/v2/highlights/1")
      .with_status(200)
      .create();

    let result = client().delete(1);
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }
}
