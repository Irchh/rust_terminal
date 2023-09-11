extern crate sdl2;

use std::path::PathBuf;
use font_kit::handle::Handle;
use font_kit::source::SystemSource;
use sdl2::Sdl;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::ttf::Font;
use sdl2::VideoSubsystem;

pub struct TermRenderer<'a> {
    pub sdl_context: &'a Sdl,
    pub ttf_context: &'a Sdl2TtfContext,
    pub font: Font<'a, 'a>,
    pub bold_font: Font<'a, 'a>,
    pub video_subsystem: VideoSubsystem,
}

impl<'a> TermRenderer<'a> {
    fn get_font_path(name: &str) -> Option<PathBuf> {
        match SystemSource::new().select_by_postscript_name(name).ok()? {
            Handle::Path { path, .. } => Some(path),
            Handle::Memory { .. } => None,
        }
    }

    pub fn new(sdl_context: &'a Sdl, ttf_context: &'a Sdl2TtfContext) -> TermRenderer<'a> {
        let hack_regular = Self::get_font_path("Hack-Regular").unwrap();
        let hack_bold = Self::get_font_path("Hack-Bold").unwrap();
        Self {
            sdl_context,
            ttf_context,
            //font: ttf_context.load_font("/usr/share/fonts/TTF/DroidSansMono.ttf", 15).unwrap(),
            font: ttf_context.load_font(hack_regular, 15).unwrap(),
            bold_font: ttf_context.load_font(hack_bold, 15).unwrap(),
            video_subsystem: sdl_context.video().unwrap(),
        }
    }
}