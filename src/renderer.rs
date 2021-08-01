extern crate sdl2;

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

pub fn create_renderer<'a>(sdl_context: &'a Sdl, ttf_context: &'a Sdl2TtfContext) -> TermRenderer<'a> {
    let ren = TermRenderer {
        sdl_context,
        ttf_context,
        //font: ttf_context.load_font("/usr/share/fonts/TTF/DroidSansMono.ttf", 15).unwrap(),
        font: ttf_context.load_font("/usr/share/fonts/TTF/Hack-Regular.ttf", 15).unwrap(),
        bold_font: ttf_context.load_font("/usr/share/fonts/TTF/Hack-Bold.ttf", 15).unwrap(),
        video_subsystem: sdl_context.video().unwrap(),
    };

    ren
}