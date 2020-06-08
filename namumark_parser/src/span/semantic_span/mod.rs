use crate::{span_list, Result, Span};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_until},
};

#[derive(Debug, PartialEq)]
pub enum SemanticSpan {
  Strong(Vec<Span>),
  Emphasis(Vec<Span>),
  Delete(Vec<Span>),
  Underline(Vec<Span>),
  Superscript(Vec<Span>),
  Subscript(Vec<Span>),
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

#[cfg(test)]
mod delete_tests {
  use crate::*;

  #[test]
  fn basic1() {
    let source = "~~Danuel~~";
    assert_eq!(
      span_list(source),
      vec![Span::Semantic(SemanticSpan::Delete(vec![Span::Inline(
        "Danuel".to_owned()
      )]))]
    );
  }

  #[test]
  fn basic2() {
    let source = "--Danuel--";
    assert_eq!(
      span_list(source),
      vec![Span::Semantic(SemanticSpan::Delete(vec![Span::Inline(
        "Danuel".to_owned()
      )]))]
    );
  }
}

#[cfg(test)]
mod emphasis_tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "''Danuel''";
    assert_eq!(
      span_list(source),
      vec![Span::Semantic(SemanticSpan::Emphasis(vec![Span::Inline(
        "Danuel".to_owned()
      )]))]
    );
  }
}

#[cfg(test)]
mod strong_tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "'''Danuel'''";
    assert_eq!(
      span_list(source),
      vec![Span::Semantic(SemanticSpan::Strong(vec![Span::Inline(
        "Danuel".to_owned()
      )]))]
    );
  }
}

#[cfg(test)]
mod subscript_tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = ",,Danuel,,";
    assert_eq!(
      span_list(source),
      vec![Span::Semantic(SemanticSpan::Subscript(vec![Span::Inline(
        "Danuel".to_owned()
      )]))]
    );
  }
}

#[cfg(test)]
mod superscript_tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "^^Danuel^^";
    assert_eq!(
      span_list(source),
      vec![Span::Semantic(SemanticSpan::Superscript(vec![
        Span::Inline("Danuel".to_owned())
      ]))]
    );
  }
}

#[cfg(test)]
mod underline_tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "__Danuel__";
    assert_eq!(
      span_list(source),
      vec![Span::Semantic(SemanticSpan::Underline(vec![Span::Inline(
        "Danuel".to_owned()
      )]))]
    );
  }
}
