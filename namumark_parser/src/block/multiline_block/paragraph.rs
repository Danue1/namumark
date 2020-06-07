use crate::{span_list, MultilineBlock, Result, EMPTY};

pub(crate) fn paragraph(input: &str) -> Result<MultilineBlock> {
  Ok((EMPTY, MultilineBlock::Paragraph(span_list(input))))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn paragraph() {
    let source = "Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Paragraph(vec![
        Span::Inline("Danuel".to_owned())
      ]))]
    );
  }
}
