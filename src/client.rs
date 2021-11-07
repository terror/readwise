use crate::common::*;

#[derive(Debug)]
pub struct Client {
  pub access_token: String,
}

impl Client {
  /// Fetch all books from a specified page
  pub fn get_books(&self, page: i64) -> Result<Vec<Book>> {
    let response = signed_request(
      &format!("/books?page={}", page),
      &self.access_token,
      Method::GET,
      None,
    )?;

    let data: BooksResponse = serde_json::from_str(&response.text()?)?;
    Ok(data.results)
  }

  /// Fetch all highlights from a specified page
  pub fn get_highlights(&self, page: i64) -> Result<Vec<Highlight>> {
    let response = signed_request(
      &format!("/highlights?page={}", page),
      &self.access_token,
      Method::GET,
      None,
    )?;

    let data: HighlightsResponse = serde_json::from_str(&response.text()?)?;
    Ok(data.results)
  }

  /// Fetch a single book by ID
  pub fn get_book(&self, id: i64) -> Result<Book> {
    let response = signed_request(
      &format!("/books/{}", id),
      &self.access_token,
      Method::GET,
      None,
    )?;

    let data: Book = serde_json::from_str(&response.text()?)?;
    Ok(data)
  }

  /// Fetch a single highlight by ID
  pub fn get_highlight(&self, id: i64) -> Result<Highlight> {
    let resp = signed_request(
      &format!("/highlights/{}", id),
      &self.access_token,
      Method::GET,
      None,
    )?;

    let response_text = resp.text()?;
    let data: Highlight = serde_json::from_str(&response_text)?;
    Ok(data)
  }

  /// Create one or more highlights and return them
  pub fn create_highlights(&self, highlights: Vec<HashMap<&str, &str>>) -> Result<Vec<Highlight>> {
    let mut body = HashMap::new();
    body.insert("highlights", highlights);

    let response = signed_request("/highlights", &self.access_token, Method::POST, Some(body))?;

    let highlight_information: Vec<HighlightCreateResponse> =
      serde_json::from_str(&response.text()?)?;

    let mut created_highlights: Vec<Highlight> = Vec::new();
    for response_item in highlight_information {
      for id in response_item.modified_highlights {
        created_highlights.push(self.get_highlight(id)?);
      }
    }

    Ok(created_highlights)
  }

  /// Update a single highlight and return it
  pub fn update_highlight(&self, id: i64, body: HashMap<&str, &str>) -> Result<Highlight> {
    let mut container = HashMap::new();
    container.insert("body", vec![body]);

    let resp = signed_request(
      &format!("/highlights/{}", id),
      &self.access_token,
      Method::PATCH,
      Some(container),
    )?;

    let response_text = &resp.text()?;
    let updated_highlight: Highlight = serde_json::from_str(response_text)?;
    Ok(updated_highlight)
  }

  /// Delete a single highlight
  pub fn delete_highlight(&self, id: i64) -> Result<()> {
    signed_request(
      &format!("/highlights/{}", id),
      &self.access_token,
      Method::DELETE,
      None,
    )?;

    Ok(())
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
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());

    let client = result.unwrap();
    assert_eq!("token", client.access_token);
  }

  #[test]
  fn test_authenticate_bad_token() {
    let _m = mock("GET", "/api/v2/auth").with_status(401).create();

    let result = auth("token");
    assert!(result.is_err(), "{}", result.err().unwrap().to_string());
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

    let result = client().get_books(1);
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn test_highlights() {
    let _m = mock("GET", "/api/v2/highlights?page=1")
      .with_status(200)
      .with_body(format!(
        r#" {{ "count": 1, "next": null, "previous": null, "results": [{}] }} "#,
        get_highlight_as_string()
      ))
      .create();

    let result = client().get_highlights(1);
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn test_single_book() {
    let _m = mock("GET", "/api/v2/books/1")
      .with_status(200)
      .with_body(get_book_as_string())
      .create();

    let result = client().get_book(1);
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn test_single_highlight() {
    let _m = mock("GET", "/api/v2/highlights/1")
      .with_status(200)
      .with_body(get_highlight_as_string())
      .create();

    let result = client().get_highlight(1);
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
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

    let result = client().create_highlights(Vec::new());
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn test_update_highlight() {
    let _m = mock("PATCH", "/api/v2/highlights/0")
      .with_status(200)
      .with_body(get_highlight_as_string())
      .create();

    let result = client().update_highlight(0, HashMap::new());
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn test_delete_highlight() {
    let _m = mock("DELETE", "/api/v2/highlights/1")
      .with_status(200)
      .create();

    let result = client().delete_highlight(1);
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }
}
