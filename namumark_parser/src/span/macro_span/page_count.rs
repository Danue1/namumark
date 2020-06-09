use crate::{MacroSpan, Result, EMPTY};
use nom::{
  bytes::complete::{tag, take_until},
  character::complete::char,
  combinator::{all_consuming, opt},
};

pub(crate) fn page_count(input: &str) -> Result<MacroSpan> {
  let (input, _) = identifier(input)?;
  let (_, namespace) = opt(parens)(input)?;
  let span = MacroSpan::PageCount(namespace.map(|namespace| namespace));

  Ok((EMPTY, span))
}

fn identifier(input: &str) -> Result {
  let (input, _) = tag("pagecount")(input)?;

  Ok((input, ()))
}

fn parens(input: &str) -> Result<&str> {
  let (input, _) = char('(')(input)?;
  let (end_input, input) = take_until(")")(input)?;
  let _ = all_consuming(char(')'))(end_input)?;

  Ok((EMPTY, input))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn pagecount() {
    let source = "[pagecount]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::PageCount(None))]
    );
  }

  #[test]
  fn pagecount_with_namespace() {
    let source = "[pagecount(file)]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::PageCount(Some("file")))]
    );
  }
}
