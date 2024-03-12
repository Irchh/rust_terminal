extern crate sdl2;

use std::path::PathBuf;
use font_kit::handle::Handle;
use font_kit::source::SystemSource;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::ttf::Font;
use sdl2::VideoSubsystem;
use crate::term::CharacterCellManager;

pub struct TermRenderer<'a> {
    width: usize,
    height: usize,
    pub font_width: u32,
    pub font_height: u32,

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

    pub fn new(sdl_context: &'a Sdl, ttf_context: &'a Sdl2TtfContext, width: usize, height: usize) -> TermRenderer<'a> {
        let hack_regular = Self::get_font_path("Hack-Regular").unwrap();
        let hack_bold = Self::get_font_path("Hack-Bold").unwrap();
        let font = ttf_context.load_font(hack_regular, 15).unwrap();
        let bold_font = ttf_context.load_font(hack_bold, 15).unwrap();

        let char_surf = font.render_char('i')
            .shaded(Color{r:0,g:0,b:0,a:0}, Color{r:0,g:0,b:0,a:0})
            .unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let this = Self {
            width,
            height,
            font_width: char_surf.width(),
            font_height: char_surf.height(),
            sdl_context,
            ttf_context,
            //font: ttf_context.load_font("/usr/share/fonts/TTF/DroidSansMono.ttf", 15).unwrap(),
            font,
            bold_font,
            video_subsystem,
        };

        this
    }

    pub fn render(&mut self, terminal_buffer: &CharacterCellManager, canvas: &mut WindowCanvas) {
        let tex_creator = canvas.texture_creator();
        let cells = &terminal_buffer.cells;

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = cells.get(y).unwrap().get(x).unwrap();
                if !cell.dirty {
                    continue;
                }
                let ch = if cell.ch == '\0' {
                    ' '
                } else {
                    cell.ch
                };

                let fgc;
                let bgc;
                if cell.inverse {
                    fgc = cell.bg_col;
                    bgc = cell.fg_col;
                } else {
                    fgc = cell.fg_col;
                    bgc = cell.bg_col;
                }

                let text_surf = if cell.bold {
                    self.bold_font.render_char(ch)
                        .shaded(fgc, bgc)
                        .unwrap()
                } else {
                    self.font.render_char(ch)
                        .shaded(fgc, bgc)
                        .unwrap()
                };

                let text_texture = text_surf.as_texture(&tex_creator).unwrap();
                let text_rect = Rect::new(0, 0, self.font_width, self.font_height);
                let real_rect = Rect::new((x * ((self.font_width) as usize)) as i32, (y * (self.font_height as usize)) as i32, self.font_width, self.font_height);
                canvas.copy(&text_texture, text_rect, real_rect).unwrap();
            }
        }
    }
}