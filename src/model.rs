use crate::common::*;

#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct BooksResponse {
  pub count:    i64,
  pub next:     Option<String>,
  pub previous: Option<String>,
  pub results:  Vec<Book>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct HighlightsResponse {
  pub count:    i64,
  pub next:     Option<String>,
  pub previous: Option<String>,
  pub results:  Vec<Highlight>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HighlightCreateResponse {
  pub id:                  i64,
  pub title:               String,
  pub auhtor:              Option<String>,
  pub category:            String,
  pub num_highlights:      i64,
  pub last_highlighted_at: Option<String>,
  pub updated:             String,
  pub cover_image_url:     String,
  pub highlights_url:      String,
  pub source_url:          Option<String>,
  pub modified_highlights: Vec<i64>,
}
