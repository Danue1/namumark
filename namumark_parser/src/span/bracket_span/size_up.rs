use super::SIZE_LEVEL;
use crate::{span_list, whitespace1, BracketSpan, Result};
use nom::bytes::complete::{is_a, tag, take};

pub(crate) fn size_up(input: &str) -> Result<BracketSpan> {
  let (input, (level, span_input)) = expect_size_up(input)?;
  let span_list = span_list(span_input);
  let span = BracketSpan::SizeUp(span_list, level.into());

  Ok((input, span))
}

// TODO(Danuel): 사이즈레벨이 2글자 이상이어도 통과하는 버그 수정
pub(crate) fn expect_size_up(input: &str) -> Result<(usize, &str)> {
  fn start(input: &str) -> Result<usize> {
    let (input, _) = tag("+")(input)?;
    let (input, level) = take(1usize)(input)?;
    let _ = is_a(SIZE_LEVEL)(level)?;
    let (input, _) = whitespace1(input)?;

    Ok((input, level.parse().unwrap()))
  };

  let (input, level) = start(input)?;

  Ok((input, (level, input)))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn basic1() {
    let source = "{{{+1 Danuel}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SizeUp(
        vec![Span::Inline("Danuel")],
        FontSizeLevel::One
      ))]
    );
  }

  #[test]
  fn basic2() {
    let source = "{{{+2 Danuel}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SizeUp(
        vec![Span::Inline("Danuel")],
        FontSizeLevel::Two
      ))]
    );
  }

  #[test]
  fn basic3() {
    let source = "{{{+3 Danuel}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SizeUp(
        vec![Span::Inline("Danuel")],
        FontSizeLevel::Three
      ))]
    );
  }

  #[test]
  fn basic4() {
    let source = "{{{+4 Danuel}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SizeUp(
        vec![Span::Inline("Danuel")],
        FontSizeLevel::Four
      ))]
    );
  }

  #[test]
  fn basic5() {
    let source = "{{{+5 Danuel}}}";
    assert_eq!(
      span_list(source),
      vec![Span::Bracket(BracketSpan::SizeUp(
        vec![Span::Inline("Danuel")],
        FontSizeLevel::Five
      ))]
    );
  }
}
