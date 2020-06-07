use crate::{MacroSpan, Result, EMPTY};
use nom::{bytes::complete::tag, combinator::all_consuming};

pub(crate) fn linebreak(input: &str) -> Result<MacroSpan> {
  let _ = all_consuming(tag("br"))(input)?;
  let span = MacroSpan::Linebreak;

  Ok((EMPTY, span))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn linebreak() {
    let source = "[br]";
    assert_eq!(span_list(source), vec![Span::Macro(MacroSpan::Linebreak)]);
  }
}
