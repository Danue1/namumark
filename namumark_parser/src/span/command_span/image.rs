use crate::{pipeline, CommandSpan, ImageOption, Result, EMPTY};
use nom::bytes::complete::{tag, take_till};

// TODO(Danuel): [[:파일:]] 문법 추가 필요
pub(crate) fn image(input: &str) -> Result<CommandSpan> {
  let (input, _) = start(input)?;
  let (input, url) = url(input)?;
  let option = option(input);
  let span = CommandSpan::Image(url.to_owned(), option);

  Ok((EMPTY, span))
}

fn start(input: &str) -> Result {
  let (input, _) = tag("파일:")(input)?;

  Ok((input, ()))
}

fn url(input: &str) -> Result<&str> {
  let (input, url) = take_till(|character| character == '|')(input)?;

  Ok((pipeline(input), url))
}

fn option(input: &str) -> ImageOption {
  let mut option: ImageOption = Default::default();

  for token in input.split('&') {
    let token_list: Vec<&str> = token.split('=').collect();
    match token_list[..] {
      ["width", value] => {
        option.width = value.into();
      }
      ["height", value] => {
        option.height = value.into();
      }
      ["align", value] => {
        option.align = value.into();
      }
      ["background_color", value] => {
        option.background_color = value.into();
      }
      _ => {}
    }
  }

  option
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn basic() {
    let source = "[[파일:a.jpg]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        Default::default()
      ))]
    );
  }

  #[test]
  fn width() {
    let source = "[[파일:a.jpg|width=200]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          width: Size::Numeric(200.0),
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn height() {
    let source = "[[파일:a.jpg|height=200]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          height: Size::Numeric(200.0),
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn height_pixel() {
    let source = "[[파일:a.jpg|height=200px]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          height: Size::Pixel(200.0),
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn align_start() {
    let source = "[[파일:a.jpg|align=start]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          align: Alignment::Start,
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn align_end() {
    let source = "[[파일:a.jpg|align=end]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          align: Alignment::End,
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn align_left() {
    let source = "[[파일:a.jpg|align=left]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          align: Alignment::Left,
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn align_center() {
    let source = "[[파일:a.jpg|align=center]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          align: Alignment::Center,
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn align_right() {
    let source = "[[파일:a.jpg|align=right]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          align: Alignment::Right,
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn background_hex() {
    let source = "[[파일:a.jpg|background=#000000]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          background_color: Color::Hex(0, 0, 0),
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn width_height() {
    let source = "[[파일:a.jpg|width=200&height=200]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          width: Size::Numeric(200.0),
          height: Size::Numeric(200.0),
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn height_width() {
    let source = "[[파일:a.jpg|height=200&width=200]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Image(
        "a.jpg".to_owned(),
        ImageOption {
          width: Size::Numeric(200.0),
          height: Size::Numeric(200.0),
          ..Default::default()
        }
      ))]
    );
  }
}
