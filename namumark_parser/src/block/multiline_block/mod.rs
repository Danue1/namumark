pub(crate) mod blockquote;
pub(crate) mod horizontal_rule;
mod indent;
pub(crate) mod list;
mod paragraph;

use crate::{Result, Span};
use blockquote::blockquote;
use horizontal_rule::horizontal_rule;
use indent::indent;
use list::list;
use nom::branch::alt;
use paragraph::paragraph;

#[derive(Debug, PartialEq)]
pub enum MultilineBlock<'a> {
  Blockquote(Vec<MultilineBlock<'a>>),
  HorizontalRule,
  Indent(Vec<MultilineBlock<'a>>),
  OrderedList(Vec<ListItem<'a>>, ListIndex<'a>),
  Paragraph(Vec<Span<'a>>),
  UnorderedList(Vec<ListItem<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum ListIndex<'a> {
  Numeric(&'a str),
  HangulChosung(&'a str),
  HangulSyllable(&'a str),
  LowerAlphabet(&'a str),
  UpperAlphabet(&'a str),
  LowerArabic(&'a str),
  UpperArabic(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct ListItem<'a>(pub Vec<MultilineBlock<'a>>);

impl<'a> ListItem<'a> {
  pub fn iter(&self) -> std::slice::Iter<MultilineBlock<'a>> {
    self.0.iter()
  }
}

impl<'a> IntoIterator for ListItem<'a> {
  type Item = MultilineBlock<'a>;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

pub(crate) fn multiline_block(input: &str) -> Result<MultilineBlock> {
  alt((list, indent, horizontal_rule, blockquote, paragraph))(input)
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
