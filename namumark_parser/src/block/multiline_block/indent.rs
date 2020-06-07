use crate::{line_with_bracket, multiline_block_list, MultilineBlock, Result};
use nom::character::complete::char;

pub(crate) fn indent(input: &str) -> Result<MultilineBlock> {
  let (input, _) = expect_indent(input)?;
  let (input, line) = line_with_bracket(input);

  Ok((input, MultilineBlock::Indent(multiline_block_list(line))))
}

fn expect_indent(input: &str) -> Result {
  let (input, _) = char(' ')(input)?;

  Ok((input, ()))
}
