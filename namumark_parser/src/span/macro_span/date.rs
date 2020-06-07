use crate::{MacroSpan, Result, EMPTY};
use nom::{bytes::complete::tag, combinator::all_consuming};

pub(crate) fn date(input: &str) -> Result<MacroSpan> {
  let _ = all_consuming(tag("date"))(input)?;
  let span = MacroSpan::Date;

  Ok((EMPTY, span))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "[date]";
    assert_eq!(span_list(source), vec![Span::Macro(MacroSpan::Date)]);
  }
}
