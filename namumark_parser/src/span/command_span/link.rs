use crate::{pipeline, span_list, CommandSpan, Result, EMPTY};
use nom::bytes::complete::take_till;

pub(crate) fn link(input: &str) -> Result<CommandSpan> {
  let (input, link) = start(input)?;
  let span_list = span_list(input);
  let span = CommandSpan::Link(span_list, link);

  Ok((EMPTY, span))
}

fn start(input: &str) -> Result<&str> {
  let (input, link) = take_till(|character| character == '|')(input)?;
  let input = pipeline(input);

  Ok((input, link))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn empty() {
    let source = "[[]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Link(vec![], ""))]
    );
  }

  #[test]
  fn a_link() {
    let source = "[[/foo]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Link(vec![], "/foo"))]
    );
  }

  #[test]
  fn a_link_with_text() {
    let source = "[[/foo|Danuel]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Link(
        vec![Span::Inline("Danuel")],
        "/foo"
      ))]
    );
  }
}
