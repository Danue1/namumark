mod multiline_block;
mod singleline_block;

use crate::Result;
use multiline_block::multiline_block;
pub use multiline_block::{ListIndex, ListItem, MultilineBlock};
use singleline_block::singleline_block;
pub use singleline_block::{HeadingLevel, SinglelineBlock};

#[derive(Debug, PartialEq)]
pub enum Block<'a> {
  Singleline(SinglelineBlock<'a>),
  Multiline(MultilineBlock<'a>),
}

pub fn block_list(mut input: &str) -> Vec<Block> {
  let mut block_list = vec![];

  while let Ok((next_input, block)) = block(input) {
    block_list.push(block);
    if next_input.is_empty() {
      break;
    }
    input = next_input;
  }

  block_list
}

fn block(input: &str) -> Result<Block> {
  if let Ok((input, block)) = singleline_block(input) {
    Ok((input, Block::Singleline(block)))
  } else {
    let (input, block) = multiline_block(input)?;

    Ok((input, Block::Multiline(block)))
  }
}

pub(crate) fn multiline_block_list(input: &str) -> Vec<MultilineBlock> {
  let mut input = input;
  let mut block_list = vec![];

  while let Ok((next_input, block)) = multiline_block(input) {
    block_list.push(block);
    if next_input.is_empty() {
      break;
    }
    input = next_input;
  }

  block_list
}
