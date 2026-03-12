use maud::{Markup, Render};

pub fn singular_or_plural<'a>(
  count: usize,
  singular: &'a str,
  plural: &'a str,
) -> Markup {
  match count {
    1 => singular.render(),
    _ => plural.render(),
  }
}
