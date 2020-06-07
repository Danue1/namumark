use crate::{MacroSpan, Result, RubyOption, EMPTY};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_till, take_until},
  character::complete::char,
  combinator::{all_consuming, opt},
};

pub(crate) fn ruby(input: &str) -> Result<MacroSpan> {
  let (input, _) = start(input)?;
  let (input, _) = end(input)?;
  if let Ok((input, word)) = word(input) {
    if let Ok((_, ruby_option)) = ruby_option(input) {
      let span = MacroSpan::Ruby(Some((word.to_owned(), ruby_option)));

      return Ok((EMPTY, span));
    }
  }

  let span = MacroSpan::Ruby(None);

  Ok((EMPTY, span))
}

fn start(input: &str) -> Result {
  let (input, _) = tag("ruby(")(input)?;

  Ok((input, ()))
}

fn end(input: &str) -> Result {
  let (end_input, input) = take_until(")")(input)?;
  let _ = all_consuming(char(')'))(end_input)?;

  Ok((input, ()))
}

fn word(input: &str) -> Result<&str> {
  let (input, word) = take_till(|character| character == ',' || character == ')')(input)?;
  let (input, _) = opt(alt((char(','), char(')'))))(input)?;

  Ok((input, word))
}

fn ruby_option(input: &str) -> Result<RubyOption> {
  let mut ruby_option: RubyOption = Default::default();
  for token in input.split(',') {
    let token_list: Vec<&str> = token.split('=').map(|token| token.trim()).collect();
    match *token_list.as_slice() {
      ["ruby", value] => {
        ruby_option.text = value.to_owned();
      }
      ["color", value] => {
        ruby_option.color = value.into();
      }
      _ => {}
    }
  }

  Ok((EMPTY, ruby_option))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn a_text() {
    let source = "[ruby(foo, ruby=bar)]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::Ruby(Some((
        "foo".to_owned(),
        RubyOption {
          text: "bar".to_owned(),
          ..Default::default()
        }
      ))))]
    );
  }

  #[test]
  fn a_text_with_color() {
    let source = "[ruby(foo, ruby=bar, color=#000000)]";
    assert_eq!(
      span_list(source),
      vec![Span::Macro(MacroSpan::Ruby(Some((
        "foo".to_owned(),
        RubyOption {
          text: "bar".to_owned(),
          color: Color::Hex(0, 0, 0)
        }
      ))))]
    );
  }
}
