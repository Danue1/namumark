mod category;
mod image;
mod link;
mod video;

use crate::{Alignment, Color, Result, Size, Span};
use category::category;
use image::image;
use link::link;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_until},
};
use video::video;

#[derive(Debug, PartialEq)]
pub enum CommandSpan<'a> {
  Category(&'a str),
  Image(&'a str, ImageOption<'a>),
  /// parent link와 child link는 아래 2개 케이스에 대해서만 작동한다
  /// parent => ../
  /// child => /foo
  Link(Vec<Span<'a>>, &'a str),
  Video(&'a str, VideoOption),
}

#[derive(Debug, Default, PartialEq)]
pub struct ImageOption<'a> {
  width: Size,
  height: Size,
  align: Alignment,
  background_color: Color<'a>,
}

#[derive(Debug, Default, PartialEq)]
pub struct VideoOption {
  platform: VideoPlatform,
  width: Size,
  height: Size,
  start: u32,
  end: u32,
}

#[derive(Debug, PartialEq)]
pub enum VideoPlatform {
  Youtube,
  KakaoTv,
  NicoVideo,
}

impl Default for VideoPlatform {
  fn default() -> Self {
    Self::Youtube
  }
}

pub(crate) fn command_span(input: &str) -> Result<CommandSpan> {
  fn start(input: &str) -> Result {
    let (input, _) = tag("[[")(input)?;

    Ok((input, ()))
  };

  fn end(input: &str) -> Result<&str> {
    let (_, line) = take_until("]]")(input)?;

    Ok((&input[line.len() + 2..], line))
  };

  let (input, _) = start(input)?;
  let (input, line) = end(input)?;
  let (_, span) = alt((image, video, category, link))(line)?;

  Ok((input, span))
}
