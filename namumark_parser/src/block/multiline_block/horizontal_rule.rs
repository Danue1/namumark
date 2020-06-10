use crate::{line, MultilineBlock, Result};
use nom::{
  bytes::complete::{take_while, take_while_m_n},
  combinator::all_consuming,
};

pub(crate) fn horizontal_rule(input: &str) -> Result<MultilineBlock> {
  let (input, line) = line(input);
  let _ = all_consuming(is_valid)(line)?;
  let block = MultilineBlock::HorizontalRule;

  Ok((input, block))
}

fn is_valid(input: &str) -> Result {
  let (input, _) = take_while(|character| character == ' ')(input)?;
  let (input, _) = take_while_m_n(4, 9, |character| character == '-')(input)?;

  Ok((input, ()))
}

pub(crate) fn starts_with_horizontal_rule(input: &str) -> bool {
  let (_, line) = line(input);

  all_consuming(is_valid)(line).is_ok()
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn level3() {
    let source = "---";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Paragraph(vec![
        Span::Inline("---")
      ]))]
    );
  }

  #[test]
  fn level4() {
    let source = "----";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::HorizontalRule)]
    );
  }

  #[test]
  fn level5() {
    let source = "-----";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::HorizontalRule)]
    );
  }

  #[test]
  fn level6() {
    let source = "------";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::HorizontalRule)]
    );
  }

  #[test]
  fn level7() {
    let source = "-------";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::HorizontalRule)]
    );
  }

  #[test]
  fn level8() {
    let source = "--------";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::HorizontalRule)]
    );
  }

  #[test]
  fn level9() {
    let source = "---------";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::HorizontalRule)]
    );
  }

  #[test]
  fn level10() {
    let source = "----------";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Paragraph(vec![
        Span::Semantic(SemanticSpan::Delete(vec![])),
        Span::Semantic(SemanticSpan::Delete(vec![])),
        Span::Inline("--")
      ]))]
    );
  }

  #[test]
  fn indented1() {
    let source = " ----";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Indent(vec![
        MultilineBlock::HorizontalRule
      ]))]
    );
  }

  #[test]
  fn indented2() {
    let source = "  ----";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Indent(vec![
        MultilineBlock::Indent(vec![MultilineBlock::HorizontalRule])
      ]))]
    );
  }

  #[test]
  fn indented3() {
    let source = "   ----";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Indent(vec![
        MultilineBlock::Indent(vec![MultilineBlock::Indent(vec![
          MultilineBlock::HorizontalRule
        ])])
      ]))]
    );
  }
}
