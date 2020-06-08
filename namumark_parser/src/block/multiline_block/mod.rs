mod blockquote;
mod horizontal_rule;
mod indent;
mod list;
mod paragraph;

use crate::{Result, Span};
use blockquote::blockquote;
use horizontal_rule::horizontal_rule;
use indent::indent;
use list::list;
use nom::branch::alt;
use paragraph::paragraph;

#[derive(Debug, PartialEq)]
pub enum MultilineBlock {
  Blockquote(Vec<MultilineBlock>),
  HorizontalRule,
  Indent(Vec<MultilineBlock>),
  OrderedList(Vec<ListItem>, ListIndex),
  Paragraph(Vec<Span>),
  UnorderedList(Vec<ListItem>),
}

#[derive(Debug, PartialEq)]
pub enum ListIndex {
  Numeric(String),
  HangulChosung(String),
  HangulSyllable(String),
  LowerAlphabet(String),
  UpperAlphabet(String),
  LowerArabic(String),
  UpperArabic(String),
}

#[derive(Debug, PartialEq)]
pub struct ListItem(pub Vec<MultilineBlock>);

impl ListItem {
  pub fn iter(&self) -> std::slice::Iter<MultilineBlock> {
    self.0.iter()
  }

  pub fn into_iter(self) -> std::vec::IntoIter<MultilineBlock> {
    self.0.into_iter()
  }
}

pub(crate) fn multiline_block(input: &str) -> Result<MultilineBlock> {
  alt((list, indent, horizontal_rule, blockquote, paragraph))(input)
}
