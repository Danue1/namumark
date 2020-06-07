use crate::{BracketSpan, Result, EMPTY};
use nom::{branch::alt, bytes::complete::tag};

pub(crate) fn syntax_highlight(input: &str) -> Result<BracketSpan> {
  let (input, _) = start(input)?;
  let (input, language) = language(input)?;
  let span = BracketSpan::SyntaxHighlight(input.to_owned(), language.to_owned());

  Ok((EMPTY, span))
}

fn start(input: &str) -> Result {
  let (input, _) = tag("#!syntax ")(input)?;

  Ok((input, ()))
}

fn language(input: &str) -> Result<&str> {
  alt((tag("javascript"), tag("rust")))(input)
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn javascript() {
    let source = "{{{#!syntax javascript}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SyntaxHighlight(
        "".to_owned(),
        "javascript".to_owned(),
      ))]
    )
  }

  #[test]
  fn javascript_with_singleline_text() {
    let source = "{{{#!syntax javascript code}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SyntaxHighlight(
        " code".to_owned(),
        "javascript".to_owned(),
      ))]
    )
  }

  #[test]
  fn javascript_with_multiline_text() {
    let source = "{{{#!syntax javascript
code
}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SyntaxHighlight(
        "\ncode\n".to_owned(),
        "javascript".to_owned(),
      ))]
    )
  }

  #[test]
  fn rust() {
    let source = "{{{#!syntax rust}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SyntaxHighlight(
        "".to_owned(),
        "rust".to_owned(),
      ))]
    )
  }

  #[test]
  fn rust_with_singleline_text() {
    let source = "{{{#!syntax rust code}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SyntaxHighlight(
        " code".to_owned(),
        "rust".to_owned(),
      ))]
    )
  }

  #[test]
  fn rust_with_multiline_text() {
    let source = "{{{#!syntax rust
code
}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SyntaxHighlight(
        "\ncode\n".to_owned(),
        "rust".to_owned(),
      ))]
    )
  }
}
