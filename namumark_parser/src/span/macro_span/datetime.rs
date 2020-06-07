use crate::{MacroSpan, Result, EMPTY};
use nom::{bytes::complete::tag, combinator::all_consuming};

pub(crate) fn datetime(input: &str) -> Result<MacroSpan> {
  let _ = all_consuming(tag("datetime"))(input)?;
  let span = MacroSpan::Datetime;

  Ok((EMPTY, span))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "[datetime]";
    assert_eq!(span_list(source), vec![Span::Macro(MacroSpan::Datetime)]);
  }
}
