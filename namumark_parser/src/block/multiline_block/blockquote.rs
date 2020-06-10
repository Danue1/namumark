use super::{indent, list, paragraph};
use crate::{linebreak, whitespace, MultilineBlock, Result};
use nom::{branch::alt, character::complete::char};

pub(crate) fn blockquote(input: &str) -> Result<MultilineBlock> {
  let _ = expect_blockquote(input)?;

  let mut index = 0;
  let mut block_list = vec![];

  for line in input
    .lines()
    .take_while(|&line| starts_with_blockquote(line))
  {
    index += line.len() + 1;
    let line = whitespace(&line[1..]);
    if let Ok((_, block)) = alt((list, indent, blockquote, paragraph))(line) {
      block_list.push(block);
    }
  }

  let input = linebreak(&input[index - 1..]);
  let block = MultilineBlock::Blockquote(block_list);

  Ok((input, block))
}

fn expect_blockquote(input: &str) -> Result {
  let (input, _) = char('>')(input)?;

  Ok((input, ()))
}

pub(crate) fn starts_with_blockquote(input: &str) -> bool {
  input.starts_with('>')
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn a_line() {
    let source = ">Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Blockquote(vec![
        MultilineBlock::Paragraph(vec![Span::Inline("Danuel")])
      ]))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = "> Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Blockquote(vec![
        MultilineBlock::Paragraph(vec![Span::Inline("Danuel")])
      ]))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = ">  Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Blockquote(vec![
        MultilineBlock::Indent(vec![MultilineBlock::Paragraph(vec![Span::Inline(
          "Danuel"
        )])])
      ]))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = ">Danuel
Danuel";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::Blockquote(vec![MultilineBlock::Paragraph(
          vec![Span::Inline("Danuel")]
        )])),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline("Danuel")]))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = ">Danuel
>Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Blockquote(vec![
        MultilineBlock::Paragraph(vec![Span::Inline("Danuel")]),
        MultilineBlock::Paragraph(vec![Span::Inline("Danuel")])
      ],))]
    )
  }
}
