use std::fmt;

pub struct ThousandsSeparated<T>(pub T);

impl<T: Into<u64> + Copy> fmt::Display for ThousandsSeparated<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", format_with_commas(self.0.into()))
  }
}

fn format_with_commas(n: u64) -> String {
  let s = n.to_string();
  let chars: Vec<char> = s.chars().collect();
  let mut result = String::new();

  for (i, ch) in chars.iter().enumerate() {
    if i > 0 && (chars.len() - i).is_multiple_of(3) {
      result.push(',');
    }
    result.push(*ch);
  }

  result
}
