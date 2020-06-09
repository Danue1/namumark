use crate::{span_list, whitespace, MacroSpan, Result, EMPTY};
use nom::{bytes::complete::take_until, character::complete::char, combinator::opt};

pub(crate) fn comment(input: &str) -> Result<MacroSpan> {
  let (input, _) = identifier(input)?;
  let (input, description) = opt(description)(input)?;
  let span_list = span_list(whitespace(input));
  let span = MacroSpan::Comment(
    span_list,
    description.map(|description| description).unwrap_or(""),
  );

  Ok((EMPTY, span))
}

fn identifier(input: &str) -> Result {
  let (input, _) = char('*')(input)?;

  Ok((input, ()))
}

fn description(input: &str) -> Result<&str> {
  let (input, description) = take_until(" ")(input)?;

  Ok((input, description))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn a_text() {
    let source = "[* Danuel]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::Comment(
        vec![Span::Inline("Danuel")],
        ""
      ))]
    );
  }

  #[test]
  fn a_text_with_a_anchor() {
    let source = "[*Foo Danuel]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::Comment(
        vec![Span::Inline("Danuel")],
        "Foo"
      ))]
    );
  }
}
