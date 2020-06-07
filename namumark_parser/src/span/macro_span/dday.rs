use crate::{MacroSpan, Result, EMPTY};
use nom::{
  bytes::complete::{tag, take_till},
  character::complete::char,
  combinator::all_consuming,
};

pub(crate) fn dday(input: &str) -> Result<MacroSpan> {
  let (input, _) = start(input)?;
  let (input, _) = end(input)?;
  let span = MacroSpan::Dday(input.to_owned());

  Ok((EMPTY, span))
}

fn start(input: &str) -> Result {
  let (input, _) = tag("dday")(input)?;

  Ok((input, ()))
}

fn end(input: &str) -> Result {
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
    let source = "[dday(2020-01-01)]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::Dday("2020-01-01".to_owned()))]
    );
  }
}