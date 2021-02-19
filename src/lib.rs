/// ! Rust wrapper for the Readwise public API. The official readwise public API
/// ! documentation can be found [here](https://readwise.io/api_deets).
use anyhow::{anyhow, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const URL: &str = "https://www.readwise.io";

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
  id:                  i64,
  title:               String,
  author:              String,
  category:            String,
  num_highlights:      i64,
  last_highlighted_at: Option<String>,
  updated:             String,
  cover_image_url:     String,
  highlights_url:      String,
  source_url:          Option<String>,
}

/// An individual highlight
#[derive(Serialize, Deserialize, Default)]
pub struct Highlight {
  id:             i64,
  text:           String,
  note:           String,
  location:       i64,
  location_type:  String,
  highlighted_at: Option<String>,
  url:            Option<String>,
  color:          String,
  updated:        String,
  books_id:       i64,
}

fn get_request_url() -> String {
  #[cfg(not(test))]
  let url = format!("{}", URL);
  #[cfg(test)]
  let url = format!("{}", mockito::server_url());
  url
}

fn get_next(n: String) -> String {
  let v: Vec<&str> = n.split("/").collect();
  v[v.len() - 1].to_string()
}

impl Client {
  /// Fetch all books
  pub fn books(&self) -> Result<Vec<Book>> {
    let mut ret: Vec<Book> = Vec::new();
    let mut next = String::from("/books");

    loop {
      let resp = signed_request(&format!("{}", next), &self.access_token)?;

      if resp.status().is_success() {
        let response_text = &resp.text()?;
        let data: BooksResponse = serde_json::from_str(&response_text)?;

        for book in data.results {
          ret.push(book);
        }

        if let Some(n) = data.next {
          next = get_next(n)
        } else {
          break;
        }
      } else {
        Err(anyhow!("Failed to fetch books with status code: {}.", resp.status()))?
      }
    }
    Ok(ret)
  }

  /// Fetch all highlights
  pub fn highlights(&self) -> Result<Vec<Highlight>> {
    let mut ret: Vec<Highlight> = Vec::new();
    let mut next = String::from("/highlights");

    loop {
      let resp = signed_request(&format!("{}", next), &self.access_token)?;

      if resp.status().is_success() {
        let response_text = &resp.text()?;
        let data: HighlightsResponse = serde_json::from_str(&response_text)?;

        for highlight in data.results {
          ret.push(highlight);
        }

        if let Some(n) = data.next {
          next = get_next(n);
        } else {
          break;
        }
      } else {
        Err(anyhow!("Failed to fetch highlights with status code: {}.", resp.status()))?
      }
    }
    Ok(ret)
  }

  /// Fetch a single book by ID
  pub fn book(&self, id: i64) -> Result<Book> {
    let resp = signed_request(&format!("/books/{}", id), &self.access_token)?;
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
    let resp = signed_request(&format!("/highlights/{}", id), &self.access_token)?;
    if resp.status().is_success() {
      let response_text = resp.text()?;
      let data: Highlight = serde_json::from_str(&response_text)?;
      Ok(data)
    } else {
      Err(anyhow!("Failed to fetch highlight with id: {}", id))
    }
  }
}

fn signed_request(url: &str, token: &str) -> Result<reqwest::blocking::Response> {
  let request_client = reqwest::blocking::Client::new();

  let resp = request_client
    .post(&format!("{}/api/v2{}/", get_request_url(), url))
    .header("Authorization", format!("Token {}", token))
    .send()?;

  Ok(resp)
}

/// Authenticate using a readwise access token
pub fn auth(access_token: &str) -> Result<Client> {
  let resp = signed_request("/auth", access_token)?;

  if resp.status().is_success() {
    Ok(Client {
      access_token: access_token.to_string(),
    })
  } else {
    Err(anyhow!("Authentication failed with status code: {}", resp.status()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockito::mock;

  fn client() -> Client {
    Client { access_token: String::new() }
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
    let _m = mock("POST", "/api/v2/auth/").with_status(204).create();

    let result = auth("token");
    assert!(result.is_ok(), result.err().unwrap().to_string());

    let client = result.unwrap();
    assert_eq!("token", client.access_token);
  }

  #[test]
  fn test_authenticate_bad_token() {
    let _m = mock("POST", "/api/v2/auth/").with_status(401).create();

    let result = auth("token");
    assert!(result.is_err(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_books() {
    let _m = mock("POST", "/api/v2/books/")
      .with_status(200)
      .with_body(format!(
        r#" {{ "count": 1, "next": null, "previous": null, "results": [{}] }} "#,
        &get_book_as_string()
      ))
      .create();

    let result = client().books();
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_highlights() {
    let _m = mock("POST", "/api/v2/highlights/")
      .with_status(200)
      .with_body(format!(
        r#" {{ "count": 1, "next": null, "previous": null, "results": [{}] }} "#,
        &get_highlight_as_string()
      ))
      .create();

    let result = client().highlights();
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_single_book() {
    let _m = mock("POST", "/api/v2/books/1/")
      .with_status(200)
      .with_body(format!("{}", &get_book_as_string()))
      .create();

    let result = client().book(1);
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }

  #[test]
  fn test_single_highlight() {
    let _m = mock("POST", "/api/v2/highlights/1/")
      .with_status(200)
      .with_body(format!("{}", &get_highlight_as_string()))
      .create();

    let result = client().highlight(1);
    assert!(result.is_ok(), result.err().unwrap().to_string());
  }
}
