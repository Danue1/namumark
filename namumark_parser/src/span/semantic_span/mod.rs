use crate::{span_list, Result, Span};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_until},
  character::complete::char,
};

#[derive(Debug, PartialEq)]
pub enum SemanticSpan<'a> {
  Delete(Vec<Span<'a>>),
  Emphasis(Vec<Span<'a>>),
  Strong(Vec<Span<'a>>),
  Subscript(Vec<Span<'a>>),
  Superscript(Vec<Span<'a>>),
  Underline(Vec<Span<'a>>),
  Linebreak,
}

pub(crate) fn semantic_span(input: &str) -> Result<SemanticSpan> {
  alt((
    strong,
    emphasis,
    delete1,
    delete2,
    underline,
    superscript,
    subscript,
    linebreak,
  ))(input)
}

pub(crate) fn starts_with_sematic_span(input: &str) -> bool {
  expect_strong(input).is_ok()
    || expect_emphasis(input).is_ok()
    || expect_delete1(input).is_ok()
    || expect_delete2(input).is_ok()
    || expect_underline(input).is_ok()
    || expect_superscript(input).is_ok()
    || expect_subscript(input).is_ok()
    || starts_with_linebreak(input)
}

macro_rules! semantic_span {
  ($name:ident, $expect_with_name:ident, $marker:expr, $variant:ident) => {
    fn $name(input: &str) -> Result<SemanticSpan> {
      let (input, span_input) = $expect_with_name(input)?;
      let span_list = span_list(span_input);

      Ok((input, SemanticSpan::$variant(span_list)))
    }

    fn $expect_with_name(input: &str) -> Result<&str> {
      const MARKER: &'static str = $marker;
      const MARKER_COUNT: usize = MARKER.len();

      fn parens(input: &str) -> Result<&str> {
        let (input, _) = tag(MARKER)(input)?;
        let (input, span_input) = take_until(MARKER)(input)?;

        Ok((&input[MARKER_COUNT..], span_input))
      };

      let (input, span_input) = parens(input)?;

      Ok((input, span_input))
    }
  };
}

semantic_span!(strong, expect_strong, "'''", Strong);
semantic_span!(emphasis, expect_emphasis, "''", Emphasis);
semantic_span!(delete1, expect_delete1, "~~", Delete);
semantic_span!(delete2, expect_delete2, "--", Delete);
semantic_span!(underline, expect_underline, "__", Underline);
semantic_span!(superscript, expect_superscript, "^^", Superscript);
semantic_span!(subscript, expect_subscript, ",,", Subscript);

fn linebreak(input: &str) -> Result<SemanticSpan> {
  let (input, _) = char('\n')(input)?;
  let span = SemanticSpan::Linebreak;

  Ok((input, span))
}

fn starts_with_linebreak(input: &str) -> bool {
  input.starts_with('\n')
}

#[cfg(test)]
mod tests {
  use crate::*;

  macro_rules! semantic_test {
    ($source:expr, $variant:ident, $name:ident) => {
      #[test]
      fn $name() {
        assert_eq!(
          span_list($source),
          vec![Span::Semantic(SemanticSpan::$variant(vec![Span::Inline(
            "Danuel"
          )]))]
        )
      }
    };
  }

  semantic_test!("~~Danuel~~", Delete, delete1);
  semantic_test!("--Danuel--", Delete, delete2);
  semantic_test!("''Danuel''", Emphasis, emphasis);
  semantic_test!("'''Danuel'''", Strong, strong);
  semantic_test!(",,Danuel,,", Subscript, subscript);
  semantic_test!("^^Danuel^^", Superscript, superscript);
  semantic_test!("__Danuel__", Underline, underline);
}
