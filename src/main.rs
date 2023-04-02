mod book;
mod constant;
mod kind;

use crate::{
  book::{gen_name_from_short_sentence, get_item_from_books, get_rand_short_sentence_from_list, get_item_book},
  kind::BookKind,
};

fn main() {
  let (cur_type, books) = get_item_book();
  let cur_detail = get_item_from_books(&books);
  let short_sentences = cur_detail.get_short_sentences().unwrap();
  let short_sentence_item = get_rand_short_sentence_from_list(&short_sentences);
  let name = gen_name_from_short_sentence(short_sentence_item);

  println!("请输入您的姓氏：");
  let mut user_last_name = String::new();
  std::io::stdin().read_line(&mut user_last_name).unwrap();
  let mut final_name = user_last_name.trim().to_string();
  final_name.push_str(name.as_str());

  println!(
    r#"
您生成的姓名是：{:?}
取自：{}
作者：{}
朝代：{}
短句是：{}
长句是：{}
  "#,
    final_name,
    BookKind::to_kind(cur_type.as_str()),
    cur_detail.author.clone().unwrap(),
    cur_detail.dynasty.clone().unwrap(),
    short_sentence_item,
    cur_detail.content.clone().unwrap()
  );
}
