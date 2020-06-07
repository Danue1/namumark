use crate::{MacroSpan, Result, EMPTY};
use nom::{branch::alt, bytes::complete::tag, combinator::all_consuming};

pub(crate) fn footnote(input: &str) -> Result<MacroSpan> {
  let _ = all_consuming(alt((tag("각주"), tag("footnote"))))(input)?;
  let span = MacroSpan::Footnote;

  Ok((EMPTY, span))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn hangul() {
    let source = "[각주]";
    assert_eq!(span_list(source), vec![Span::Macro(MacroSpan::Footnote)]);
  }

  #[test]
  fn alphabet() {
    let source = "[footnote]";
    assert_eq!(span_list(source), vec![Span::Macro(MacroSpan::Footnote)]);
  }
}
