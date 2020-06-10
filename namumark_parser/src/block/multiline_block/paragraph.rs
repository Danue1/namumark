use super::{
  super::singleline_block::{comment::starts_with_comment, open_heading::starts_with_open_heading},
  blockquote::starts_with_blockquote,
  horizontal_rule::starts_with_horizontal_rule,
  list::starts_with_list,
};
use crate::{line_with_bracket, linebreak, span_list, whitespace, MultilineBlock, Result};

pub(crate) fn paragraph(input: &str) -> Result<MultilineBlock> {
  let (input, line) = line(input);
  let span_list = span_list(line);
  let block = MultilineBlock::Paragraph(span_list);

  Ok((input, block))
}

fn line(input: &str) -> (&str, &str) {
  let mut index = 0;
  while index < input.len() {
    if let Some(slice) = input.get(index..) {
      if starts_with_other_block(slice) {
        break;
      } else if slice.starts_with('\n') {
        index += 1;
      } else {
        loop {
          if let Some(next_line) = input.get(index..) {
            if let Some(line) = next_line.lines().next() {
              if let Some(bracket_index) = line.find("{{{") {
                index += bracket_index;
                let (_, bracket_line) = line_with_bracket(&input[index..]);
                index += bracket_line.len();
              } else {
                index += line.len();
                break;
              }
            } else {
              index += 1;
              break;
            }
          } else {
            break;
          }
        }
      }
    } else {
      break;
    }
  }

  (linebreak(&input[index..]), whitespace(&input[..index]))
}

fn starts_with_other_block(input: &str) -> bool {
  starts_with_blockquote(input)
    || starts_with_horizontal_rule(input)
    || starts_with_list(input)
    || starts_with_comment(input)
    || starts_with_open_heading(input)
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn paragraph() {
    let source = "Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::Paragraph(vec![
        Span::Inline("Danuel")
      ]))]
    );
  }
}
