use crate::{MacroSpan, Result, EMPTY};
use nom::{
  bytes::complete::{tag, take_till},
  character::complete::char,
  combinator::{all_consuming, opt},
};

// TODO(Danuel): value 문법 확인 후 구현 추가 필요
pub(crate) fn include(input: &str) -> Result<MacroSpan> {
  let (input, _) = identifier(input)?;
  let (input, _) = parens(input)?;
  let (_, namespace) = namespace(input)?;
  let span = MacroSpan::Include(namespace);

  Ok((EMPTY, span))
}

fn identifier(input: &str) -> Result {
  let (input, _) = tag("include")(input)?;

  Ok((input, ()))
}

fn parens(input: &str) -> Result {
  let (input, _) = char('(')(input)?;
  let (end_input, input) = take_till(|character| character == ')')(input)?;
  let _ = all_consuming(char(')'))(end_input)?;

  Ok((input, ()))
}

fn namespace(input: &str) -> Result<&str> {
  let (input, namespace) = take_till(|character| character == ',')(input)?;
  let (input, _) = opt(char(','))(input)?;

  Ok((input, namespace))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn empty() {
    let source = "[include()]";
    assert_eq!(span_list(source), vec![Span::Macro(MacroSpan::Include(""))]);
  }

  #[test]
  fn namespace() {
    let source = "[include(foo)]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::Include("foo"))]
    );
  }
}
