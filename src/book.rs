use rand::thread_rng;
use rand::Rng;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Book {
  pub author: Option<String>,
  pub content: Option<String>,
  pub title: Option<String>,
  pub book: Option<String>,
  pub dynasty: Option<String>,
}

impl Book {
  pub fn get_short_sentences(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let replace_char_to_empty = Regex::new(r#"[，。；<、>！p《》（）]"#)?;

    if let Some(content) = &self.content {
      let list = replace_char_to_empty
        .split(content)
        .map(|sentences| sentences.trim().to_string())
        .collect::<Vec<String>>();
      return Ok(list);
    }
    Ok(vec![])
  }
}

pub type Books = Vec<Book>;

pub fn get_item_book() -> (String, Books) {
  use crate::constant::{JSON_BASE, JSON_NAMES};
  use serde_json::from_str;
  use std::{fs::read_to_string, path::Path};

  let index = thread_rng().gen_range(0..JSON_NAMES.len());
  let cur_type = JSON_NAMES.get(index).unwrap();

  let json_path = Path::new(JSON_BASE).join(format!("{}.json", cur_type));
  let item_lists = read_to_string(json_path).unwrap();
  let books: Books = from_str(item_lists.as_str()).unwrap();

  (cur_type.to_string(), books)
}

pub fn get_item_from_books(books: &Books) -> &Book {
  let rand_index = rand::thread_rng().gen_range(0..books.len());
  let cur = books.get(rand_index).unwrap();
  cur
}

pub fn get_rand_short_sentence_from_list<'a>(list: &'a Vec<String>) -> &'a String {
  let rand_index = thread_rng().gen_range(0..list.len());
  let cur_short_sentence = list.get(rand_index).unwrap();
  cur_short_sentence
}

pub fn gen_name_from_short_sentence(sentence: &String) -> String {
  let len = if sentence.len() > 1 { 2 } else { 1 };
  let mut name = String::new();

  for _ in 0..len {
    let index = rand::thread_rng().gen_range(0..sentence.chars().count());
    name.push(sentence.chars().nth(index).unwrap())
  }

  name
}
