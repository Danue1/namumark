mod color;
mod folding;
mod inline;
mod size_down;
mod size_up;
mod syntax_highlight;

use crate::{Color, FontSizeLevel, MultilineBlock, Result, Span};
use color::color;
use folding::folding;
use inline::inline;
use nom::{branch::alt, bytes::complete::tag, error::ErrorKind, Err};
use size_down::size_down;
use size_up::size_up;
use syntax_highlight::syntax_highlight;

const SIZE_LEVEL: &str = "12345";

#[derive(Debug, PartialEq)]
pub enum BracketSpan<'a> {
  Color(Vec<Span<'a>>, Color<'a>),
  Folding(Vec<MultilineBlock<'a>>),
  Inline(&'a str),
  SizeDown(Vec<Span<'a>>, FontSizeLevel),
  SizeUp(Vec<Span<'a>>, FontSizeLevel),
  SyntaxHighlight(&'a str, &'a str),
}

pub(crate) fn bracket_span(input: &str) -> Result<BracketSpan> {
  let _ = tag("{{{")(input)?;
  let (input, bracket) = line_with_bracket(input);
  if let Some(bracket) = bracket {
    let (_, span) = alt((size_up, size_down, color, folding, syntax_highlight, inline))(
      &bracket[3..bracket.len() - 3],
    )?;

    Ok((input, span))
  } else {
    Err(Err::Error((input, ErrorKind::Verify)))
  }
}

pub(crate) fn starts_with_bracket_span(input: &str) -> bool {
  let (_, line) = line_with_bracket(input);

  line.is_some()
}

fn line_with_bracket(input: &str) -> (&str, Option<&str>) {
  const START: &str = "{{{";
  const END: &str = "}}}";

  let mut range: Option<(usize, usize)> = None;
  let mut index = 0;
  let mut index_stack = vec![];

  while index < input.len() {
    match input.get(index..index + START.len()) {
      Some(START) => {
        index_stack.push(index);
        index += START.len();
      }
      Some(END) => {
        if let Some(start_offset) = index_stack.pop() {
          range = range
            .map(|(start, _)| (std::cmp::min(start, start_offset), index))
            .or_else(|| Some((start_offset, index)));
          index += END.len();
          if index_stack.is_empty() {
            break;
          }
        } else {
          break;
        }
      }
      _ => {
        index += 1;
      }
    }
  }

  match range {
    Some((0, end)) => (&input[end + START.len()..], Some(&input[..end + END.len()])),
    Some((start, _)) => (&input[..start], None),
    _ => (input, None),
  }
}
