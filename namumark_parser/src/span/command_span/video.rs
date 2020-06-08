use crate::{CommandSpan, Result, VideoOption, VideoPlatform, EMPTY};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_till, take_until},
  character::complete::char,
  combinator::all_consuming,
};

pub(crate) fn video(input: &str) -> Result<CommandSpan> {
  let (input, platform) = identifier(input)?;
  let (input, _) = parens(input)?;
  let (url, option) = center(input, platform)?;
  let span = CommandSpan::Video(url.to_owned(), option);

  Ok((EMPTY, span))
}

fn identifier(input: &str) -> Result<VideoPlatform> {
  macro_rules! platform {
    ($identifier:expr, $name:ident, $variant:ident) => {
      let $name = |input| {
        let (input, _) = tag($identifier)(input)?;

        Ok((input, VideoPlatform::$variant))
      };
    };
  }

  platform!("youtube", youtube, Youtube);
  platform!("kakaotv", kakaotv, KakaoTv);
  platform!("nicovideo", nicovideo, NicoVideo);

  let (input, platform) = alt((youtube, kakaotv, nicovideo))(input)?;
  let (input, _) = char('(')(input)?;

  Ok((input, platform))
}

fn center(input: &str, platform: VideoPlatform) -> Result<VideoOption> {
  let (input, url) = take_till(|character| character == ',')(input)?;
  let mut option = VideoOption {
    platform,
    ..Default::default()
  };

  for token in input.split(',') {
    let token_list: Vec<&str> = token.split('=').collect();
    match token_list[..] {
      ["width", value] => {
        option.width = value.into();
      }
      ["height", value] => {
        option.height = value.into();
      }
      ["start", value] => {
        option.start = value.parse().unwrap();
      }
      ["end", value] => {
        option.end = value.parse().unwrap();
      }
      _ => {}
    }
  }

  Ok((url, option))
}

fn parens(input: &str) -> Result {
  let (end_input, input) = take_until(")")(input)?;
  let (end_input, _) = char(')')(end_input)?;
  let _ = all_consuming(|input: &str| -> Result { Ok((input, ())) })(end_input);

  Ok((input, ()))
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn youtube() {
    let source = "[[youtube(danuel)]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Video(
        "danuel".to_owned(),
        VideoOption {
          platform: VideoPlatform::Youtube,
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn kakaotv() {
    let source = "[[kakaotv(danuel)]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Video(
        "danuel".to_owned(),
        VideoOption {
          platform: VideoPlatform::KakaoTv,
          ..Default::default()
        }
      ))]
    );
  }

  #[test]
  fn nicovideo() {
    let source = "[[nicovideo(danuel)]]";
    assert_eq!(
      span_list(source),
      vec![Span::Command(CommandSpan::Video(
        "danuel".to_owned(),
        VideoOption {
          platform: VideoPlatform::NicoVideo,
          ..Default::default()
        }
      ))]
    );
  }
}
