pub struct BookKind;

impl BookKind {
  pub fn to_kind(kind: &str) -> String {
    match kind {
      "chuci" => "楚辞".to_string(),
      "cifu" => "辞赋".to_string(),
      "gushi" => "古诗".to_string(),
      "shijing" => "诗经".to_string(),
      "songci" => "宋词".to_string(),
      "tangshi" => "唐诗".to_string(),
      "yuefu" => "乐府".to_string(),
      _ => panic!("暂不支持的类型"),
    }
  }
}
