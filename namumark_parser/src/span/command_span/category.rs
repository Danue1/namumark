use crate::{CommandSpan, Result, EMPTY};
use nom::bytes::complete::tag;

pub(crate) fn category(input: &str) -> Result<CommandSpan> {
  let (input, _) = identifier(input)?;
  let span = CommandSpan::Category(input.to_owned());

  Ok((EMPTY, span))
}

fn identifier(input: &str) -> Result {
  let (input, _) = tag("분류:")(input)?;

  Ok((input, ()))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn empty() {
    let source = "[[분류:]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Category("".to_owned()))]
    )
  }

  #[test]
  fn namespace() {
    let source = "[[분류:foo]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Category("foo".to_owned()))]
    )
  }
}
