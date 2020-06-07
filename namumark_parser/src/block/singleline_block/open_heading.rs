use crate::{line, span_list, Result, SinglelineBlock};
use nom::{
  character::complete::char,
  multi::{count, fold_many_m_n},
};

pub(crate) fn open_heading(input: &str) -> Result<SinglelineBlock> {
  let (input, line) = line(input);
  let (line, level) = start(line)?;
  let (line, _) = end(line, level)?;
  let span_list = span_list(line);
  let block = SinglelineBlock::OpenHeading(span_list, level.into());

  Ok((input, block))
}

fn start(input: &str) -> Result<usize> {
  let (input, level) = fold_many_m_n(1, 6, char('='), 0, |level, _| level + 1)(input)?;
  let (input, _) = char(' ')(input)?;

  Ok((input, level))
}

fn end(input: &str, level: usize) -> Result {
  let marker_position = input.len() - level - 1;
  let (input, tail) = (&input[..marker_position], &input[marker_position..]);
  let (tail, _) = char(' ')(tail)?;
  let _ = count(char('='), level)(tail)?;

  Ok((input, ()))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn level_1() {
    let source = "= Danuel =";
    assert_eq!(
      parse(source),
      vec![Block::Singleline(SinglelineBlock::OpenHeading(
        vec![Span::Inline("Danuel".to_owned())],
        HeadingLevel::One
      ))]
    );
  }

  #[test]
  fn level_2() {
    let source = "== Danuel ==";
    assert_eq!(
      parse(source),
      vec![Block::Singleline(SinglelineBlock::OpenHeading(
        vec![Span::Inline("Danuel".to_owned())],
        HeadingLevel::Two
      ))]
    );
  }

  #[test]
  fn level_3() {
    let source = "=== Danuel ===";
    assert_eq!(
      parse(source),
      vec![Block::Singleline(SinglelineBlock::OpenHeading(
        vec![Span::Inline("Danuel".to_owned())],
        HeadingLevel::Three
      ))]
    );
  }

  #[test]
  fn level_4() {
    let source = "==== Danuel ====";
    assert_eq!(
      parse(source),
      vec![Block::Singleline(SinglelineBlock::OpenHeading(
        vec![Span::Inline("Danuel".to_owned())],
        HeadingLevel::Four
      ))]
    );
  }

  #[test]
  fn level_5() {
    let source = "===== Danuel =====";
    assert_eq!(
      parse(source),
      vec![Block::Singleline(SinglelineBlock::OpenHeading(
        vec![Span::Inline("Danuel".to_owned())],
        HeadingLevel::Five
      ))]
    );
  }

  #[test]
  fn level_6() {
    let source = "====== Danuel ======";
    assert_eq!(
      parse(source),
      vec![Block::Singleline(SinglelineBlock::OpenHeading(
        vec![Span::Inline("Danuel".to_owned())],
        HeadingLevel::Six
      ))]
    );
  }
}
