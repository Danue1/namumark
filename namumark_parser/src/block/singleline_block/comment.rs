use crate::{line, Result, SinglelineBlock};
use nom::bytes::complete::tag;

pub(crate) fn comment(input: &str) -> Result<SinglelineBlock> {
  let (input, line) = line(input);
  let (line, _) = start(line)?;
  let block = SinglelineBlock::Comment(line);

  Ok((input, block))
}

fn start(input: &str) -> Result {
  let (input, _) = tag("##")(input)?;

  Ok((input, ()))
}

pub(crate) fn starts_with_comment(input: &str) -> bool {
  input.starts_with("##")
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
