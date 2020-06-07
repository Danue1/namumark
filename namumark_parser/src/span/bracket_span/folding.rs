use crate::{multiline_block_list, BracketSpan, Result, EMPTY};
use nom::bytes::complete::tag;

pub(crate) fn folding(input: &str) -> Result<BracketSpan> {
  let (input, _) = tag("#!folding ")(input)?;
  let block_list = multiline_block_list(input);
  let span = BracketSpan::Folding(block_list);

  Ok((EMPTY, span))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn folding() {
    let source = "{{{#!folding Danuel}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::Folding(vec![
        MultilineBlock::Paragraph(vec![Span::Inline("Danuel".to_owned())])
      ]))]
    )
  }
}
