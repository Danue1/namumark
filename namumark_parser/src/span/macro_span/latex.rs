use crate::{MacroSpan, Result, EMPTY};
use nom::{bytes::complete::tag, combinator::verify};

pub(crate) fn latex(input: &str) -> Result<MacroSpan> {
  let (input, _) = start(input)?;
  let (input, _) = end(input)?;
  let span = MacroSpan::Latex(input.to_owned());

  Ok((EMPTY, span))
}

fn start(input: &str) -> Result {
  let (input, _) = tag("math(")(input)?;

  Ok((input, ()))
}

fn end(input: &str) -> Result {
  let _ = verify(
    |input| -> Result<&str> { Ok((input, input)) },
    |input: &str| input.ends_with(input),
  )(input)?;

  Ok((&input[..input.len() - 1], ()))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "[math(Danuel)]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::Latex("Danuel".to_owned()))]
    );
  }
}
