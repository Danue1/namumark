use crate::{Result, EMPTY};
use nom::character::complete::char;

pub(crate) fn slice_by(character: char) -> impl Fn(&str) -> &str {
  move |input: &str| -> &str {
    if input.starts_with(character) {
      &input[1..]
    } else {
      input
    }
  }
}

pub(crate) fn pipeline(input: &str) -> &str {
  slice_by('|')(input)
}

pub(crate) fn linebreak(input: &str) -> &str {
  slice_by('\n')(input)
}

pub(crate) fn whitespace(input: &str) -> &str {
  slice_by(' ')(input)
}

pub(crate) fn whitespace1(input: &str) -> Result {
  let (input, _) = char(' ')(input)?;

  Ok((input, ()))
}

pub(crate) fn line(input: &str) -> (&str, &str) {
  if let Some(index) = input.find('\n') {
    (&input[index + 1..], &input[..index])
  } else {
    (EMPTY, input)
  }
}

pub(crate) fn line_with_bracket(input: &str) -> (&str, &str) {
  const START: &'static str = "{{{";
  const END: &'static str = "}}}";

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
            let input = &input[index..];
            if let Some(bracket_index) = input.find(START) {
              if let Some(_) = input[..bracket_index].find('\n') {
                break;
              }
            }
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
    Some((0, end)) => (&input[end + START.len()..], &input[..end + END.len()]),
    Some((start, _)) => (&input[start..], &input[..start]),
    _ => ("", input),
  }
}
