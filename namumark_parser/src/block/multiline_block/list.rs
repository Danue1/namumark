use crate::{
  line_with_bracket, linebreak, multiline_block_list, whitespace, ListIndex, ListItem,
  MultilineBlock, Result,
};
use nom::{
  branch::alt,
  bytes::complete::{is_a, tag},
  character::complete::char,
  combinator::opt,
};

const UNORDERED_LIST: &'static str = " *";

pub(crate) fn list(input: &str) -> Result<MultilineBlock> {
  alt((unordered_list, ordered_list))(input)
}

fn starts_with_list(input: &str) -> bool {
  !input.starts_with(' ') || starts_with_unordered_list(input) || starts_with_ordered_list(input)
}

fn unordered_list(input: &str) -> Result<MultilineBlock> {
  let (input, _) = expect_unorder_list(input)?;
  let (mut input, item) = list_item_line(input);

  let mut item_list = vec![];
  if !item.is_empty() {
    item_list.push(list_item(item))
  }

  while starts_with_list(input) {
    if let Ok((next_input, _)) = expect_unorder_list(input) {
      let (next_input, cursor) = list_start(next_input)?;
      if cursor.is_some() {
        break;
      }
      let (next_input, item) = list_item_line(next_input);
      item_list.push(list_item(item));
      input = next_input;
    } else {
      break;
    }
  }

  let block = MultilineBlock::UnorderedList(item_list);

  Ok((input, block))
}

fn expect_unorder_list(input: &str) -> Result {
  let (input, _) = tag(UNORDERED_LIST)(input)?;

  Ok((input, ()))
}

fn starts_with_unordered_list(input: &str) -> bool {
  input.starts_with(UNORDERED_LIST)
}

fn ordered_list(input: &str) -> Result<MultilineBlock> {
  alt((
    ordered_list_numeric,
    ordered_list_lower_alphabet,
    ordered_list_upper_alphabet,
    ordered_list_lower_arabic,
    ordered_list_upper_arabic,
    ordered_list_hangul_chosung,
    ordered_list_hangul_syllable,
  ))(input)
}

fn starts_with_ordered_list(input: &str) -> bool {
  starts_with_ordered_list_numeric(input)
    || starts_with_ordered_list_lower_alphabet(input)
    || starts_with_ordered_list_upper_alphabet(input)
    || starts_with_ordered_list_lower_arabic(input)
    || starts_with_ordered_list_upper_arabic(input)
    || starts_with_ordered_list_hangul_chosung(input)
    || starts_with_ordered_list_hangul_syllable(input)
}

// TODO(Danuel): 함수를 재사용하도록 최적화 할 필요 있음
macro_rules! ordered_list_type {
  ($marker:expr, $variant:ident, $name:ident, $expect_with_name:ident, $starts_with_name:ident) => {
    fn $name(input: &str) -> Result<MultilineBlock> {
      let (input, _) = $expect_with_name(input)?;
      let (input, cursor) = list_start(input)?;
      let (mut input, item) = list_item_line(input);

      let mut item_list = vec![];
      if !item.is_empty() {
        item_list.push(list_item(item))
      }

      while starts_with_list(input) {
        if let Ok((next_input, _)) = $expect_with_name(input) {
          let (next_input, cursor) = list_start(next_input)?;
          if cursor.is_some() {
            break;
          }
          let (next_input, item) = list_item_line(next_input);
          item_list.push(list_item(item));
          input = next_input;
        } else {
          break;
        }
      }

      let list_index = ListIndex::$variant(cursor.unwrap_or_else(|| "1").to_owned());
      let block = MultilineBlock::OrderedList(item_list, list_index);

      Ok((input, block))
    }

    fn $expect_with_name(input: &str) -> Result {
      let (input, _) = tag($marker)(input)?;

      Ok((input, ()))
    }

    fn $starts_with_name(input: &str) -> bool {
      input.starts_with($marker)
    }
  };
}

ordered_list_type!(
  " 1.",
  Numeric,
  ordered_list_numeric,
  expect_ordered_list_numeric,
  starts_with_ordered_list_numeric
);
ordered_list_type!(
  " a.",
  LowerAlphabet,
  ordered_list_lower_alphabet,
  expect_ordered_list_lower_alphabet,
  starts_with_ordered_list_lower_alphabet
);
ordered_list_type!(
  " A.",
  UpperAlphabet,
  ordered_list_upper_alphabet,
  expect_ordered_list_upper_alphabet,
  starts_with_ordered_list_upper_alphabet
);
ordered_list_type!(
  " i.",
  LowerArabic,
  ordered_list_lower_arabic,
  expect_ordered_list_lower_arabic,
  starts_with_ordered_list_lower_arabic
);
ordered_list_type!(
  " I.",
  UpperArabic,
  ordered_list_upper_arabic,
  expect_ordered_list_upper_arabic,
  starts_with_ordered_list_upper_arabic
);
ordered_list_type!(
  " ㄱ.",
  HangulChosung,
  ordered_list_hangul_chosung,
  expect_ordered_list_hangul_chosung,
  starts_with_ordered_list_hangul_chosung
);
ordered_list_type!(
  " 가.",
  HangulSyllable,
  ordered_list_hangul_syllable,
  expect_ordered_list_hangul_syllable,
  starts_with_ordered_list_hangul_syllable
);

fn list_item(input: &str) -> ListItem {
  ListItem(multiline_block_list(input))
}

fn list_start(input: &str) -> Result<Option<&str>> {
  opt(|input| -> Result<&str> {
    let (input, _) = char('#')(input)?;

    is_a("0123456789abcdefghijklmnopqrstuvwxyz")(input)
  })(input)
}

fn list_item_line(input: &str) -> (&str, &str) {
  let mut index = 0;
  while index < input.len() {
    if input[index..].starts_with("{{{") {
      let (_, line) = line_with_bracket(&input[index..]);
      index += line.len();
    } else if input[index..].starts_with('\n') {
      break;
    } else {
      index += 1;
    }
  }

  (linebreak(&input[index..]), whitespace(&input[..index]))
}

#[cfg(test)]
mod unordered_list_tests {
  use crate::*;

  #[test]
  fn a_line() {
    let source = " *Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::UnorderedList(vec![
        ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
          "Danuel".to_owned()
        )])])
      ]))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = " * Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::UnorderedList(vec![
        ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
          "Danuel".to_owned()
        )])])
      ]))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = " *  Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::UnorderedList(vec![
        ListItem(vec![MultilineBlock::Indent(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("Danuel".to_owned())])
        ])])
      ]))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = " *Danuel
Danuel";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::UnorderedList(vec![ListItem(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("Danuel".to_owned())])
        ])])),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline(
          "Danuel".to_owned()
        )]))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = " *Danuel
 *Danuel";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::UnorderedList(vec![
        ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
          "Danuel".to_owned()
        )])]),
        ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
          "Danuel".to_owned()
        )])])
      ]))]
    )
  }
}

#[cfg(test)]
mod ordered_list_hangul_chosung_tests {
  use crate::*;

  #[test]
  fn a_line() {
    let source = " ㄱ.foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::HangulChosung("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = " ㄱ. foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::HangulChosung("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = " ㄱ.  foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Indent(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("foo".to_owned())])
        ])])],
        ListIndex::HangulChosung("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_singleline_bracket() {
    let source = " ㄱ.foo {{{bar}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("bar".to_owned()))
        ])])],
        ListIndex::HangulChosung("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_multiline_bracket() {
    let source = " ㄱ.foo {{{
bar
}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("\nbar\n".to_owned()))
        ])])],
        ListIndex::HangulChosung("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_specific_index() {
    let source = " ㄱ.#4 foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::HangulChosung("4".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = " ㄱ.foo
bar";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::OrderedList(
          vec![ListItem(vec![MultilineBlock::Paragraph(vec![
            Span::Inline("foo".to_owned())
          ])])],
          ListIndex::HangulChosung("1".to_owned())
        )),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline(
          "bar".to_owned()
        )],))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = " ㄱ.foo
 ㄱ.bar";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "foo".to_owned()
          )])],),
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "bar".to_owned()
          )])])
        ],
        ListIndex::HangulChosung("1".to_owned())
      ))]
    )
  }
}

#[cfg(test)]
mod ordered_list_hangul_syllable_tests {
  use crate::*;

  #[test]
  fn a_line() {
    let source = " 가.foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::HangulSyllable("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = " 가. foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::HangulSyllable("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = " 가.  foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Indent(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("foo".to_owned())])
        ])])],
        ListIndex::HangulSyllable("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_specific_index() {
    let source = " 가.#4 foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::HangulSyllable("4".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_singleline_bracket() {
    let source = " 가.foo {{{bar}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("bar".to_owned()))
        ])])],
        ListIndex::HangulSyllable("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_multiline_bracket() {
    let source = " 가.foo {{{
bar
}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("\nbar\n".to_owned()))
        ])])],
        ListIndex::HangulSyllable("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = " 가.foo
bar";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::OrderedList(
          vec![ListItem(vec![MultilineBlock::Paragraph(vec![
            Span::Inline("foo".to_owned())
          ])])],
          ListIndex::HangulSyllable("1".to_owned())
        )),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline(
          "bar".to_owned()
        )],))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = " 가.foo
 가.bar";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "foo".to_owned()
          )])],),
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "bar".to_owned()
          )])])
        ],
        ListIndex::HangulSyllable("1".to_owned())
      ))]
    )
  }
}

#[cfg(test)]
mod unordered_list_lower_alphabet {
  use crate::*;

  #[test]
  fn a_line() {
    let source = " a.foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::LowerAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = " a. foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::LowerAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = " a.  foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Indent(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("foo".to_owned())])
        ])])],
        ListIndex::LowerAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_specific_index() {
    let source = " a.#4 foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::LowerAlphabet("4".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_singleline_bracket() {
    let source = " a.foo {{{bar}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("bar".to_owned()))
        ])])],
        ListIndex::LowerAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_multiline_bracket() {
    let source = " a.foo {{{
bar
}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("\nbar\n".to_owned()))
        ])])],
        ListIndex::LowerAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = " a.foo
bar";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::OrderedList(
          vec![ListItem(vec![MultilineBlock::Paragraph(vec![
            Span::Inline("foo".to_owned())
          ])])],
          ListIndex::LowerAlphabet("1".to_owned())
        )),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline(
          "bar".to_owned()
        )],))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = " a.foo
 a.bar";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "foo".to_owned()
          )])],),
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "bar".to_owned()
          )])])
        ],
        ListIndex::LowerAlphabet("1".to_owned())
      ))]
    )
  }
}

#[cfg(test)]
mod unordered_list_upper_alphabet {
  use crate::*;

  #[test]
  fn a_line() {
    let source = " A.foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::UpperAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = " A. foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::UpperAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = " A.  foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Indent(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("foo".to_owned())])
        ])])],
        ListIndex::UpperAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_specific_index() {
    let source = " A.#4 foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::UpperAlphabet("4".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_singleline_bracket() {
    let source = " A.foo {{{bar}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("bar".to_owned()))
        ])])],
        ListIndex::UpperAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_multiline_bracket() {
    let source = " A.foo {{{
bar
}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("\nbar\n".to_owned()))
        ])])],
        ListIndex::UpperAlphabet("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = " A.foo
bar";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::OrderedList(
          vec![ListItem(vec![MultilineBlock::Paragraph(vec![
            Span::Inline("foo".to_owned())
          ])])],
          ListIndex::UpperAlphabet("1".to_owned())
        )),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline(
          "bar".to_owned()
        )],))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = " A.foo
 A.bar";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "foo".to_owned()
          )])],),
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "bar".to_owned()
          )])])
        ],
        ListIndex::UpperAlphabet("1".to_owned())
      ))]
    )
  }
}

#[cfg(test)]
mod unordered_list_lower_arabic_tests {
  use crate::*;

  #[test]
  fn a_line() {
    let source = " i.foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::LowerArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = " i. foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::LowerArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = " i.  foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Indent(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("foo".to_owned())])
        ])])],
        ListIndex::LowerArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_specific_index() {
    let source = " i.#4 foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::LowerArabic("4".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_singleline_bracket() {
    let source = " i.foo {{{bar}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("bar".to_owned()))
        ])])],
        ListIndex::LowerArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_multiline_bracket() {
    let source = " i.foo {{{
bar
}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("\nbar\n".to_owned()))
        ])])],
        ListIndex::LowerArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = " i.foo
bar";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::OrderedList(
          vec![ListItem(vec![MultilineBlock::Paragraph(vec![
            Span::Inline("foo".to_owned())
          ])])],
          ListIndex::LowerArabic("1".to_owned())
        )),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline(
          "bar".to_owned()
        )],))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = " i.foo
 i.bar";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "foo".to_owned()
          )])],),
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "bar".to_owned()
          )])])
        ],
        ListIndex::LowerArabic("1".to_owned())
      ))]
    )
  }
}

#[cfg(test)]
mod unordered_list_upper_alphabet_tests {
  use crate::*;

  #[test]
  fn a_line() {
    let source = " I.foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::UpperArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = " I. foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::UpperArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = " I.  foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Indent(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("foo".to_owned())])
        ])])],
        ListIndex::UpperArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_specific_index() {
    let source = " I.#4 foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::UpperArabic("4".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_singleline_bracket() {
    let source = " I.foo {{{bar}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("bar".to_owned()))
        ])])],
        ListIndex::UpperArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_multiline_bracket() {
    let source = " I.foo {{{
bar
}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("\nbar\n".to_owned()))
        ])])],
        ListIndex::UpperArabic("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = " I.foo
bar";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::OrderedList(
          vec![ListItem(vec![MultilineBlock::Paragraph(vec![
            Span::Inline("foo".to_owned())
          ])])],
          ListIndex::UpperArabic("1".to_owned())
        )),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline(
          "bar".to_owned()
        )],))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = " I.foo
 I.bar";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "foo".to_owned()
          )])],),
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "bar".to_owned()
          )])])
        ],
        ListIndex::UpperArabic("1".to_owned())
      ))]
    )
  }
}

#[cfg(test)]
mod unordered_list_numeric {
  use crate::*;

  #[test]
  fn a_line() {
    let source = " 1.foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::Numeric("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_space() {
    let source = " 1. foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::Numeric("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_a_indent() {
    let source = " 1.  foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Indent(vec![
          MultilineBlock::Paragraph(vec![Span::Inline("foo".to_owned())])
        ])])],
        ListIndex::Numeric("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_singleline_bracket() {
    let source = " 1.foo {{{bar}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("bar".to_owned()))
        ])])],
        ListIndex::Numeric("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_specific_index() {
    let source = " 1.#4 foo";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo".to_owned())
        ])])],
        ListIndex::Numeric("4".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_with_multiline_bracket() {
    let source = " 1.foo {{{
bar
}}}";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![ListItem(vec![MultilineBlock::Paragraph(vec![
          Span::Inline("foo ".to_owned()),
          Span::Bracket(BracketSpan::Inline("\nbar\n".to_owned()))
        ])])],
        ListIndex::Numeric("1".to_owned())
      ))]
    )
  }

  #[test]
  fn a_line_and_a_text() {
    let source = " 1.foo
bar";
    assert_eq!(
      parse(source),
      vec![
        Block::Multiline(MultilineBlock::OrderedList(
          vec![ListItem(vec![MultilineBlock::Paragraph(vec![
            Span::Inline("foo".to_owned())
          ])])],
          ListIndex::Numeric("1".to_owned())
        )),
        Block::Multiline(MultilineBlock::Paragraph(vec![Span::Inline(
          "bar".to_owned()
        )],))
      ]
    )
  }

  #[test]
  fn a_line_and_a_line() {
    let source = " 1.foo
 1.bar";
    assert_eq!(
      parse(source),
      vec![Block::Multiline(MultilineBlock::OrderedList(
        vec![
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "foo".to_owned()
          )])],),
          ListItem(vec![MultilineBlock::Paragraph(vec![Span::Inline(
            "bar".to_owned()
          )])])
        ],
        ListIndex::Numeric("1".to_owned())
      ))]
    )
  }
}
