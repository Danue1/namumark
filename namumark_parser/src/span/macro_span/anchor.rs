use crate::{MacroSpan, Result, EMPTY};
use nom::{
  bytes::complete::{tag, take_till},
  character::complete::char,
  combinator::all_consuming,
};

pub(crate) fn anchor(input: &str) -> Result<MacroSpan> {
  let (input, _) = identifier(input)?;
  let (input, _) = parens(input)?;
  let span = MacroSpan::Anchor(input.to_owned());

  Ok((EMPTY, span))
}

fn identifier(input: &str) -> Result {
  let (input, _) = tag("anchor")(input)?;

  Ok((input, ()))
}

fn parens(input: &str) -> Result {
  let (input, _) = char('(')(input)?;
  let (end_input, input) = take_till(|character| character == ')')(input)?;
  let _ = all_consuming(char(')'))(end_input)?;

  Ok((input, ()))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "[anchor(foo)]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::Anchor("foo".to_owned()))]
    );
  }
}
