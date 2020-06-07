mod block;
mod constants;
mod span;
mod utils;

use block::multiline_block_list;
pub use block::{
  block_list as parse, Block, HeadingLevel, ListIndex, ListItem, MultilineBlock, SinglelineBlock,
};
use constants::EMPTY;
use span::span_list;
pub use span::{
  Alignment, BracketSpan, Color, CommandSpan, ImageOption, MacroSpan, RubyOption, SemanticSpan,
  Size, SizeLevel, Span, VideoOption, VideoPlatform,
};
use utils::{line, line_with_bracket, linebreak, pipeline, whitespace, whitespace1};

type Result<'a, T = ()> = nom::IResult<&'a str, T>;