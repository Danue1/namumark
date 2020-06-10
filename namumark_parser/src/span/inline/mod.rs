use super::{starts_with_bracket_span, starts_with_sematic_span};
use crate::Result;

pub(crate) fn inline(input: &str) -> Result<&str> {
  let mut index = 0;
  while index < input.len() {
    if let Some(slice) = input.get(index..) {
      if starts_with_span(slice) {
        break;
      }
      index += slice.chars().next().unwrap().len_utf8();
    } else {
      break;
    }
  }
  let (input, inline) = (&input[index..], &input[..index]);

  Ok((input, inline))
}

fn starts_with_span(input: &str) -> bool {
  starts_with_sematic_span(input) || starts_with_bracket_span(input)
}

#[cfg(test)]
mod tests {
  use super::super::*;

  #[test]
  fn basic() {
    let source = "Danuel";
    assert_eq!(span_list(source), vec![Span::Inline("Danuel")]);
  }
}
