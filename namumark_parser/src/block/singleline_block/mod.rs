mod closed_heading;
pub(crate) mod comment;
pub(crate) mod open_heading;

use crate::{Result, Span};
use closed_heading::closed_heading;
use comment::comment;
use nom::branch::alt;
use open_heading::open_heading;

#[derive(Debug, PartialEq)]
pub enum SinglelineBlock<'a> {
  OpenHeading(Vec<Span<'a>>, HeadingLevel),
  ClosedHeading(Vec<Span<'a>>, HeadingLevel),
  Comment(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum HeadingLevel {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
}

impl From<usize> for HeadingLevel {
  fn from(source: usize) -> HeadingLevel {
    use HeadingLevel::*;

    match source {
      1 => One,
      2 => Two,
      3 => Three,
      4 => Four,
      5 => Five,
      6 => Six,
      _ => std::unreachable!(),
    }
  }
}

pub(crate) fn singleline_block(input: &str) -> Result<SinglelineBlock> {
  alt((open_heading, closed_heading, comment))(input)
}
