mod age;
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

use crate::{Color, Result, Span};
use age::age;
use comment::comment;
use date::date;
use datetime::datetime;
use dday::dday;
use footnote::footnote;
use include::include;
use latex::latex;
use linebreak::linebreak;
use nom::{branch::alt, bytes::complete::take_until, character::complete::char};
use page_count::page_count;
use ruby::ruby;
use table_of_contents::table_of_contents;

#[derive(Debug, PartialEq)]
pub enum MacroSpan {
  Linebreak,
  Latex(String),
  Date,
  Datetime,
  PageCount(Option<String>),
  Include(String),
  TableOfContents,
  Age(String),
  Dday(String),
  Ruby(Option<(String, RubyOption)>),
  Comment(Vec<Span>, String),
  Footnote,
}

#[derive(Debug, Default, PartialEq)]
pub struct RubyOption {
  color: Color,
  text: String,
}

pub(crate) fn macro_span(input: &str) -> Result<MacroSpan> {
  fn start(input: &str) -> Result {
    let (input, _) = char('[')(input)?;

    Ok((input, ()))
  };

  fn end(input: &str) -> Result<&str> {
    let (_, line) = take_until("]")(input)?;

    Ok((&input[line.len() + 1..], line))
  };

  let (input, _) = start(input)?;
  let (input, line) = end(input)?;
  let (_, span) = alt((
    footnote,
    linebreak,
    date,
    datetime,
    page_count,
    include,
    table_of_contents,
    latex,
    age,
    dday,
    ruby,
    comment,
  ))(line)?;

  Ok((input, span))
}
