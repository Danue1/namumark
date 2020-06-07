use crate::{span_list, whitespace1, BracketSpan, Color, Result, EMPTY};
use nom::bytes::complete::{is_a, tag, take};

pub(crate) fn color(input: &str) -> Result<BracketSpan> {
  let (input, (code, span_input)) = expect_color(input)?;
  let span_list = span_list(span_input);
  let span = BracketSpan::Color(span_list, Color::Raw(code.to_owned()));

  Ok((input, span))
}

pub(crate) fn expect_color(input: &str) -> Result<(&str, &str)> {
  const LEFT: &'static str = "#";
  const CODE: &'static str = "0123456789abcdefABCDEF";

  fn start(input: &str) -> Result<&str> {
    let (input, _) = tag(LEFT)(input)?;
    let (input, level) = take(6usize)(input)?;
    let _ = is_a(CODE)(level)?;
    let (input, _) = whitespace1(input)?;

    Ok((input, level))
  };

  let (input, code) = start(input)?;

  Ok((EMPTY, (code, input)))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "{{{#000000 Danuel}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::Color(
        vec![Span::Inline("Danuel".to_owned())],
        Color::Raw("000000".to_owned())
      ))]
    );
  }
}
