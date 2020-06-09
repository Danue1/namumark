use crate::{line, Result, SinglelineBlock};
use nom::character::complete::char;

pub(crate) fn comment(input: &str) -> Result<SinglelineBlock> {
  let (input, line) = line(input);
  let (line, _) = start(line)?;

  Ok((input, SinglelineBlock::Comment(line)))
}

fn start(input: &str) -> Result {
  let (input, _) = char('#')(input)?;
  let (input, _) = char('#')(input)?;

  Ok((input, ()))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn comment() {
    let source = "##Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Singleline(SinglelineBlock::Comment("Danuel"))]
    );
  }
}
