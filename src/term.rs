#![allow(unused_variables, dead_code)]
use rust_ansi::term::TermInterface;
use sdl2::pixels::Color;
use crate::text_area::{BG_COL, BOLD_COL_ARR, COL_ARR, FG_COL};

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
            cell_manager: CharacterCellManager { cells: vec![vec![CharacterCell { ch: ' ', fg_col: FG_COL, bg_col: BG_COL, bold: false, inverse: false, dirty: true, }; width]; height] },
            curr_fg_col: FG_COL,
            curr_bg_col: BG_COL,
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

    fn set_cell(&mut self, x: usize, y: usize, ch: char) {
        if x > self.width || y > self.height {
            eprintln!("Trying to print outside the screen to position {}:{}, this is an error", x, y);
            return;
        }
        self.cell_manager.cells[self.y as usize-1][self.x as usize-1].ch = ch;
        self.cell_manager.cells[self.y as usize-1][self.x as usize-1].fg_col = self.curr_fg_col;
        self.cell_manager.cells[self.y as usize-1][self.x as usize-1].bg_col = self.curr_bg_col;
        self.cell_manager.cells[self.y as usize-1][self.x as usize-1].bold = self.curr_is_bold;
        self.cell_manager.cells[self.y as usize-1][self.x as usize-1].inverse = self.curr_inverse;
        self.cell_manager.cells[self.y as usize-1][self.x as usize-1].dirty = true;
    }

    pub fn default_cell(&self) -> CharacterCell {
        CharacterCell {
            ch: ' ',
            fg_col: self.curr_fg_col,
            bg_col: self.curr_bg_col,
            bold: self.curr_is_bold,
            inverse: self.curr_inverse,
            dirty: true,
        }
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
        for ch in s.chars() {
            match ch {
                '\n' => {
                    self.goto_x(1);
                    self.move_y(1);
                }
                '\r' => {
                    self.goto_x(1);
                }
                '\x08' => {
                    if self.x == 1 {
                        return;
                    }
                    self.x -= 1;
                    self.set_cell(self.x as usize, self.y as usize, ' ');
                }
                _ => {
                    self.set_cell(self.x as usize, self.y as usize, ch);
                    self.move_x(1);
                }
            }
        }
    }

    fn goto_x(&mut self, x: usize) {
        self.x = x as isize;
    }

    fn goto_y(&mut self, y: usize) {
        self.y = y as isize;
    }

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
            self.scroll_up(self.y.wrapping_sub(self.height as isize) as usize);
            self.y = self.height as isize;
        }
    }

    fn erase_in_display(&mut self, n: usize) {
        let temp_y = self.y;
        match n {
            0 => {
                self.erase_in_line(n);
                if self.y < self.height as isize {
                    self.y += 1;
                }
                for y in self.y..=(self.height as isize) {
                    self.y = y;
                    self.erase_in_line(2);
                }
            }
            1 => {
                self.erase_in_line(n);
                if self.y > 1 {
                    self.y -= 1;
                }
                for y in 1..self.y {
                    self.y = y;
                    self.erase_in_line(2);
                }
            }
            2 | 3 => /* Clear screen */ {
                for y in 1..=self.height {
                    self.y = y as isize;
                    self.erase_in_line(2);
                }
            }
            _ => {
                panic!("Unknown ED {}", n)
            }
        }
        self.y = temp_y;
    }

    fn erase_in_line(&mut self, n: usize) {
        match n {
            0 => {
                for x in self.x..(self.width as isize) {
                    self.set_cell(x as usize, self.y as usize, ' ');
                }
            }
            1 => {
                for x in 1..=self.x {
                    self.set_cell(x as usize, self.y as usize, ' ');
                }
            }
            2 => {
                for x in 1..=self.width {
                    self.set_cell(x, self.y as usize, ' ');
                }
            }
            _ => {}
        }
    }

    fn scroll_up(&mut self, n: usize) {
        println!("Scrolling up {}", n);
        for _ in 0..n {
            self.cell_manager.cells.remove(0);
            self.cell_manager.cells.push(vec![self.default_cell(); self.width]);
        }

        for lines in self.cell_manager.cells.iter_mut() {
            for cell in lines.iter_mut() {
                cell.dirty = true;
            }
        }
    }

    fn scroll_down(&mut self, n: usize) {
        println!("Scrolling down {}", n);
        for _ in 0..n {
            self.cell_manager.cells.pop();
            self.cell_manager.cells.insert(0, vec![self.default_cell(); self.width]);
        }

        for lines in self.cell_manager.cells.iter_mut() {
            for cell in lines.iter_mut() {
                cell.dirty = true;
            }
        }
    }

    fn il(&mut self, n: usize) {
        todo!()
    }

    fn select_graphics_rendition(&mut self, mut n: Vec<usize>) {
        let mut iter = n.iter_mut();
        while let Some(&mut mode) = iter.next() {
            match mode {
                // Reset text attributes
                0 => self.reset(),
                1 => self.curr_is_bold = true,
                30..=37 => {
                    if self.curr_is_bold {
                        self.curr_fg_col = BOLD_COL_ARR[mode - 30];
                    } else {
                        self.curr_fg_col = COL_ARR[mode - 30];
                    }
                }
                40..=47 => self.curr_bg_col = COL_ARR[mode - 40],
                38 | 48 => {
                    match iter.next().expect("Missing color type") {
                        2 => {
                            let r = iter.next().expect("No red specified");
                            let g = iter.next().expect("No green specified");
                            let b = iter.next().expect("No blue specified");
                            if mode == 38 {
                                self.curr_fg_col = Color::RGB(*r as u8, *g as u8, *b as u8);
                            } else {
                                self.curr_bg_col = Color::RGB(*r as u8, *g as u8, *b as u8);
                            }
                        }
                        5 => {
                            let mut c = *iter.next().unwrap();
                            match c {
                                0..=7 => {
                                    if mode == 38 {
                                        self.curr_fg_col = COL_ARR[c]
                                    } else {
                                        self.curr_bg_col = COL_ARR[c]
                                    }
                                }
                                8..=15 => {
                                    if mode == 38 {
                                        self.curr_fg_col = BOLD_COL_ARR[c-8]
                                    } else {
                                        self.curr_bg_col = BOLD_COL_ARR[c-8]
                                    }
                                }
                                16..=231 => {
                                    c -= 16;
                                    let index_r = c / 36;
                                    let index_g = (c % 36) / 6;
                                    let index_b = c % 6;

                                    let r = 55 + if index_r > 0 {index_r*40} else {0} as u8;
                                    let g = 55 + if index_g > 0 {index_g*40} else {0} as u8;
                                    let b = 55 + if index_b > 0 {index_b*40} else {0} as u8;

                                    if mode == 38 {
                                        self.curr_fg_col = Color::RGB(r,g,b);
                                        print!("FG ");
                                    } else {
                                        self.curr_bg_col = Color::RGB(r,g,b);
                                        print!("BG ");
                                    }
                                    print!("\x1b[38;2;{};{};{}m", self.curr_fg_col.r, self.curr_fg_col.g, self.curr_fg_col.b);
                                    print!("\x1b[48;2;{};{};{}m", self.curr_bg_col.r, self.curr_bg_col.g, self.curr_bg_col.b);
                                    //print!("\x1b[{};2;{};{};{}m", a, r, g, b);
                                    println!("COLOR\x1b[m: {:x}, {:x}, {:x}", r, g, b)
                                }
                                _ => {
                                    let c = (0x8+(c-232)*0xA) as u8;
                                    if mode == 38 {
                                        self.curr_fg_col = Color::RGB(c,c,c);
                                    } else {
                                        self.curr_bg_col = Color::RGB(c,c,c);
                                    }
                                    println!("RGB COLOR: {:x}, {:x}, {:x}", c, c, c)
                                }
                            }
                        }
                        _ => panic!("Wrong color mode")
                    }
                }
                39 => self.curr_fg_col = FG_COL,
                49 => self.curr_bg_col = BG_COL,
                90..=97 => {
                    self.curr_fg_col = BOLD_COL_ARR[mode - 90];
                }
                100..=107 => {
                    self.curr_fg_col = BOLD_COL_ARR[mode - 100];
                }
                _ => {
                    eprintln!("Unknown SGR: {}", mode);
                }
            }
        }
    }

    fn decstbm(&mut self, top: usize, bot: usize) {
        //todo!("decstbm {} {}", top, bot)
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
        println!("Set title to: {:?}", title);
    }

    fn unknown_osc(&mut self, s: String) {
        eprintln!("Warning: Unknown OSC code: {s:?}")
    }

    fn unknown(&mut self, s: String) {
        todo!()
    }
}