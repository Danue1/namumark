mod age;
mod anchor;
mod comment;
mod date;
mod datetime;
mod dday;
mod footnote;
mod include;
mod latex;
mod linebreak;
mod page_count;
mod ruby;
mod table_of_contents;

use crate::{line, Color, Result, Span};
use age::age;
use anchor::anchor;
use comment::comment;
use date::date;
use datetime::datetime;
use dday::dday;
use footnote::footnote;
use include::include;
use latex::latex;
use linebreak::linebreak;
use nom::{branch::alt, bytes::complete::take_till, character::complete::char};
use page_count::page_count;
use ruby::ruby;
use table_of_contents::table_of_contents;

#[derive(Debug, PartialEq)]
pub enum MacroSpan<'a> {
  Age(&'a str),
  Anchor(&'a str),
  Comment(Vec<Span<'a>>, &'a str),
  Date,
  Datetime,
  Dday(&'a str),
  Footnote,
  Include(&'a str),
  Latex(&'a str),
  Linebreak,
  PageCount(Option<&'a str>),
  Ruby(Option<(&'a str, RubyOption<'a>)>),
  TableOfContents,
}

#[derive(Debug, Default, PartialEq)]
pub struct RubyOption<'a> {
  color: Color<'a>,
  text: &'a str,
}

pub(crate) fn macro_span(input: &str) -> Result<MacroSpan> {
  fn parens(input: &str) -> Result<&str> {
    let (input, _) = char('[')(input)?;
    let (input, line) = take_till(|character| character == ']')(input)?;
    let (input, _) = char(']')(input)?;

    Ok((input, line))
  };

  let (input, line) = parens(input)?;
  let (_, span) = alt((
    footnote,
    linebreak,
    date,
    datetime,
    page_count,
    include,
    table_of_contents,
    anchor,
    latex,
    age,
    dday,
    ruby,
    comment,
  ))(line)?;

  Ok((input, span))
}

pub(crate) fn starts_with_macro_span(input: &str) -> bool {
  let (_, input) = line(input);

  input.starts_with('[') && input.find(']').is_some() && macro_span(input).is_ok()
}
