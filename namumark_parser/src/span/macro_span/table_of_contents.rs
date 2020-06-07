use crate::{MacroSpan, Result, EMPTY};
use nom::{branch::alt, bytes::complete::tag, combinator::all_consuming};

pub(crate) fn table_of_contents(input: &str) -> Result<MacroSpan> {
  let _ = all_consuming(alt((tag("목차"), tag("tableofcontents"))))(input)?;
  let span = MacroSpan::TableOfContents;

  Ok((EMPTY, span))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn hangul() {
    let source = "[목차]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::TableOfContents)]
    );
  }

  #[test]
  fn alphabet() {
    let source = "[tableofcontents]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::TableOfContents)]
    );
  }
}
