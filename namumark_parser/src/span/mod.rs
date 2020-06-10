mod bracket_span;
mod command_span;
mod inline;
mod macro_span;
mod semantic_span;

use super::constants::EMPTY;
use crate::{should_be_empty, Result};
use bracket_span::bracket_span;
pub use bracket_span::BracketSpan;
use command_span::command_span;
pub use command_span::{CommandSpan, ImageOption, VideoOption, VideoPlatform};
use inline::inline;
use macro_span::macro_span;
pub use macro_span::{MacroSpan, RubyOption};
use nom::{
  bytes::complete::{tag, take},
  character::complete::char,
  combinator::all_consuming,
  number::complete::float,
};
use semantic_span::semantic_span;
pub use semantic_span::SemanticSpan;

#[derive(Debug, PartialEq)]
pub enum Span<'a> {
  Semantic(SemanticSpan<'a>),
  Bracket(BracketSpan<'a>),
  Macro(MacroSpan<'a>),
  Command(CommandSpan<'a>),
  Inline(&'a str),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Size {
  Auto,
  Numeric(f32),
  Pixel(f32),
  Rem(f32),
  Percent(f32),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Alignment {
  Auto,
  Start,
  End,
  Left,
  Center,
  Right,
}

#[derive(Debug, PartialEq)]
pub enum FontSizeLevel {
  One,
  Two,
  Three,
  Four,
  Five,
}

#[derive(Debug, PartialEq)]
pub enum Color<'a> {
  Hex(u8, u8, u8),
  Rgb(u8, u8, u8),
  Hsl(u8, u8, u8),
  Raw(&'a str),
}

impl From<usize> for FontSizeLevel {
  fn from(source: usize) -> FontSizeLevel {
    use FontSizeLevel::*;

    match source {
      1 => One,
      2 => Two,
      3 => Three,
      4 => Four,
      5 => Five,
      _ => std::unreachable!(),
    }
  }
}

impl From<&FontSizeLevel> for usize {
  fn from(source: &FontSizeLevel) -> Self {
    use FontSizeLevel::*;

    match source {
      One => 1,
      Two => 2,
      Three => 3,
      Four => 4,
      Five => 5,
    }
  }
}

impl Default for Size {
  fn default() -> Self {
    Self::Auto
  }
}

impl From<&str> for Size {
  fn from(source: &str) -> Self {
    fn numeric(input: &str) -> Result<f32> {
      float(input)
    };

    let is_match = |pattern, input| -> Result {
      let _ = all_consuming(pattern)(input)?;

      Ok((EMPTY, ()))
    };

    match numeric(source) {
      Ok((input, numeric)) => {
        if let Ok(_) = is_match(tag("px"), input) {
          Size::Pixel(numeric)
        } else if let Ok(_) = is_match(tag("rem"), input) {
          Size::Rem(numeric)
        } else if let Ok(_) = is_match(tag("%"), input) {
          Size::Percent(numeric)
        } else {
          Size::Numeric(numeric)
        }
      }
      _ => Default::default(),
    }
  }
}

impl<'a> Default for Color<'a> {
  fn default() -> Self {
    Self::Hex(0, 0, 0)
  }
}

impl<'a> From<&str> for Color<'a> {
  // TODO(Danuel): RGB, HSL 문법 추가
  fn from(source: &str) -> Self {
    fn hex<'a>(input: &str) -> Result<Color<'a>> {
      let (input, _) = char('#')(input)?;
      let (input, r) = take(2usize)(input)?;
      let (input, g) = take(2usize)(input)?;
      let (input, b) = take(2usize)(input)?;
      let _ = should_be_empty(input)?;

      let r: u8 = r.parse().unwrap();
      let g: u8 = g.parse().unwrap();
      let b: u8 = b.parse().unwrap();
      let color = Color::Hex(r, g, b);

      Ok((EMPTY, color))
    };

    if let Ok((_, color)) = hex(source) {
      color
    } else {
      Default::default()
    }
  }
}

impl Default for Alignment {
  fn default() -> Self {
    Self::Auto
  }
}

impl From<&str> for Alignment {
  fn from(source: &str) -> Self {
    match source {
      "start" => Alignment::Start,
      "end" => Alignment::End,
      "left" => Alignment::Left,
      "center" => Alignment::Center,
      "right" => Alignment::Right,
      _ => Alignment::default(),
    }
  }
}

pub fn span_list(mut input: &str) -> Vec<Span> {
  let mut list = vec![];
  while !input.is_empty() {
    match span(input) {
      Ok((next_input, span)) => {
        list.push(span);
        input = next_input;
      }
      _ => break,
    }
  }

  list
}

fn span(input: &str) -> Result<Span> {
  if let Ok((input, span)) = semantic_span(input) {
    Ok((input, Span::Semantic(span)))
  } else if let Ok((input, span)) = bracket_span(input) {
    Ok((input, Span::Bracket(span)))
  } else if let Ok((input, span)) = command_span(input) {
    Ok((input, Span::Command(span)))
  } else if let Ok((input, span)) = macro_span(input) {
    Ok((input, Span::Macro(span)))
  } else {
    let (input, inline) = inline(input)?;

    Ok((input, Span::Inline(inline)))
  }
}
