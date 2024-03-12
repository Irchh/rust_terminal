#![allow(unused_variables, dead_code)]
use rust_ansi::term::TermInterface;
use sdl2::pixels::Color;
use crate::text_area::{BG_COL, FG_COL};

#[derive(Copy, Clone)]
pub struct CharacterCell {
    pub ch: char,
    pub fg_col: Color,
    pub bg_col: Color,
    pub bold: bool,
    pub inverse: bool,
    pub dirty: bool,
}

impl CharacterCell {
    pub fn new(ch: char, fg_col: Color, bg_col: Color, bold: bool, inverse: bool, dirty: bool) -> Self {
        Self {
            ch,
            fg_col,
            bg_col,
            bold,
            inverse,
            dirty,
        }
    }
}

impl Default for CharacterCell {
    fn default() -> Self {
        Self::new(' ', Color::WHITE, Color::BLACK, false, false, true)
    }
}

pub struct CharacterCellManager {
    pub cells: Vec<Vec<CharacterCell>>,
}

pub struct Terminal {
    x: isize,
    y: isize,
    width: usize,
    height: usize,
    cell_manager: CharacterCellManager,

    // Current settings applied to any printed text
    curr_fg_col: Color,
    curr_bg_col: Color,
    curr_is_bold: bool,
    curr_inverse: bool,

    // TODO: Move to renderer since it's not terminal logic
    cursor_visible: bool,
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            x: 1,
            y: 1,
            width,
            height,
            cell_manager: CharacterCellManager { cells: vec![vec![CharacterCell::default(); width]; height] },
            curr_fg_col: Color::WHITE,
            curr_bg_col: Color::BLACK,
            curr_is_bold: false,
            curr_inverse: false,
            cursor_visible: true,
        }
    }

    fn reset(&mut self) {
        self.curr_fg_col = FG_COL;
        self.curr_bg_col = BG_COL;
        self.curr_is_bold = false;
        self.curr_inverse = false;
        self.cursor_visible = true;
    }
}

impl TermInterface<CharacterCellManager> for Terminal {
    fn framebuffer(&self) -> &CharacterCellManager {
        &self.cell_manager
    }

    fn completed_render(&mut self) {
        for row in self.cell_manager.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.dirty = false;
            }
        }
    }

    fn write(&mut self, s: String) {
        for c in s.chars() {
            self.cell_manager.cells[self.y as usize-1][self.x as usize-1].ch = c;
            self.cell_manager.cells[self.y as usize-1][self.x as usize-1].fg_col = self.curr_fg_col;
            self.cell_manager.cells[self.y as usize-1][self.x as usize-1].bg_col = self.curr_bg_col;
            self.cell_manager.cells[self.y as usize-1][self.x as usize-1].bold = self.curr_is_bold;
            self.cell_manager.cells[self.y as usize-1][self.x as usize-1].inverse = self.curr_inverse;
            self.cell_manager.cells[self.y as usize-1][self.x as usize-1].dirty = true;
            self.move_x(1);
        }
    }

    fn goto_x(&mut self, x: usize) { self.x = x as isize; }

    fn goto_y(&mut self, y: usize) { self.y = y as isize; }

    fn move_x(&mut self, x: isize) {
        // TODO: Should this overflow to next line or stop at the end?
        // TODO: Handle negative values
        self.x = self.x.wrapping_add(x);
        if self.x >= self.width as isize {
            self.x = 1;
            self.move_y(1);
        }
    }

    fn move_y(&mut self, y: isize) {
        // TODO: Handle line scrolling
        // TODO: Handle negative values
        self.y = self.y.wrapping_add(y);
        if self.y > self.height as isize {
            self.scroll_up(self.height.wrapping_sub(self.y as usize));
            self.y = self.height as isize;
            self.move_y(1);
        }
    }

    fn erase_in_display(&mut self, n: usize) {
        todo!()
    }

    fn erase_in_line(&mut self, n: usize) {
        todo!()
    }

    fn scroll_up(&mut self, n: usize) {
        todo!()
    }

    fn scroll_down(&mut self, n: usize) {
        todo!()
    }

    fn il(&mut self, n: usize) {
        todo!()
    }

    fn select_graphics_rendition(&mut self, n: Vec<usize>) {
        for mode in n {
            match mode {
                // Reset text attributes
                0 => self.reset(),
                _ => {}
            }
        }
    }

    fn decstbm(&mut self, top: usize, bot: usize) {
        todo!()
    }

    fn decslrm(&mut self, left: usize, right: usize) {
        todo!()
    }

    fn dectcem(&mut self, show: bool) { self.cursor_visible = show; }

    fn device_status_report(&mut self) -> (usize, usize) {
        todo!()
    }

    fn unknown_csi(&mut self, s: String) {
        eprintln!("Warning: Unknown CSI code: {s:?}")
    }

    fn set_title(&mut self, title: String) {
        todo!()
    }

    fn unknown_osc(&mut self, s: String) {
        eprintln!("Warning: Unknown OSC code: {s:?}")
    }

    fn unknown(&mut self, s: String) {
        todo!()
    }
}