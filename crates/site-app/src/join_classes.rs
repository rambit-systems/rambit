use std::collections::HashSet;

#[allow(dead_code)]
pub trait JoinClasses {
  fn join_classes(self) -> String;
}

impl<I: IntoIterator<Item = S>, S: AsRef<str>> JoinClasses for I {
  fn join_classes(self) -> String {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for item in self {
      for class in item.as_ref().split_whitespace().map(ToOwned::to_owned) {
        if seen.insert(class.clone()) {
          result.push(class);
        }
      }
    }

    result.join(" ")
  }
}
