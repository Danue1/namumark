use crate::{BracketSpan, Result, EMPTY};

pub(crate) fn inline(input: &str) -> Result<BracketSpan> {
  let span = BracketSpan::Inline(input);

  Ok((EMPTY, span))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "{{{Danuel}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::Inline("Danuel"))]
    );
  }
}
