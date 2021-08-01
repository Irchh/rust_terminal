extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::renderer::TermRenderer;
use sdl2::rect::Rect;
use std::borrow::Borrow;
use std::ops::Add;
use std::fmt::{Display, Formatter, Error};
use std::ffi::CString;
use nix::sys::ptrace::cont;

struct Cell {
    ch: char,
    fg_col: Color,
    bg_col: Color,
    bold: bool,
    dirty: bool,
}

impl Clone for Cell {
    fn clone(&self) -> Cell {
        Cell {
            ch: self.ch,
            fg_col: self.fg_col.clone(),
            bg_col: self.bg_col.clone(),
            bold: self.bold,
            dirty: self.dirty
        }
    }

    fn clone_from(&mut self, source: &Self) {
        todo!()
    }
}

impl Cell {
    pub fn from(c: Cell) -> Cell {
        Cell {
            ch: c.ch,
            fg_col: c.fg_col,
            bg_col: c.bg_col,
            bold: c.bold,
            dirty: true
        }
    }
}

pub struct TextArea {
    width: usize,
    height: usize,
    font_width: u32,
    font_height: u32,
    cells: Vec<Cell>,

    x: u32,
    y: u32,
    buf_str: String,

    curr_fg_col: Color,
    curr_bg_col: Color,
    fg_is_default: bool,
    curr_is_bold: bool,
    inverse: bool,

    top_margin: u32,
    bottom_margin: u32,
}

enum AnsiType {
    SS2, // Single Shift 2
    SS3, // Single Shift 3
    DCS, // Device Control String
    CSI, // Control Sequence Introducer
    ST,  // String Terminator
    OSC, // Operating System Command
    RIS, // Reset to Initial State

    // These three can be ignored (after parsing), as they are usually application specific
    SOS, // Start of String
    PM,  // Privacy Message
    APC, // Application Program Command

    Unknown,
}

impl AnsiType {
    pub fn from(ch: char) -> AnsiType {
        use crate::text_area::AnsiType::*;
        match ch {
            'N' => { SS2 }
            'O' => { SS3 }
            'P' => { DCS }
            '[' => { CSI }
            '\\' => { ST }
            ']' => { OSC }
            'X' => { SOS }
            '*' => { PM }
            '_' => { APC }
            'c' => { RIS }
            _ => { Unknown }
        }
    }
}

impl Display for AnsiType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let _ = match self {
            AnsiType::SS2 => {f.write_str("SS2")}
            AnsiType::SS3 => {f.write_str("SS3")}
            AnsiType::DCS => {f.write_str("DCS")}
            AnsiType::CSI => {f.write_str("CSI")}
            AnsiType::ST => {f.write_str("ST")}
            AnsiType::OSC => {f.write_str("OSC")}
            AnsiType::RIS => {f.write_str("RIS")}
            AnsiType::SOS => {f.write_str("SOS")}
            AnsiType::PM => {f.write_str("PM")}
            AnsiType::APC => {f.write_str("APC")}
            AnsiType::Unknown => {f.write_str("Unknown")}
        };
        Ok(())
    }
}

const COL_ARR: [Color; 8] = [
    Color {r: 0x23, g: 0x26, b: 0x27, a: 0},     // BLACK
    Color {r: 0xed, g: 0x15, b: 0x15, a: 0},   // RED
    Color {r: 0x11, g: 0xd1, b: 0x16, a: 0},   // GREEN
    Color {r: 0xf6, g: 0x74, b: 0x00, a: 0}, // YELLOW
    Color {r: 0x1d, g: 0x99, b: 0xf3, a: 0},   // BLUE
    Color {r: 0x9b, g: 0x59, b: 0xb6, a: 0}, // MAGENTA
    Color {r: 0x1a, g: 0xbc, b: 0x9c, a: 0}, // CYAN
    Color {r: 0xfc, g: 0xfc, b: 0xfc, a: 0}, // WHITE
];
const BOLD_COL_ARR: [Color; 8] = [
    Color {r: 0x7f, g: 0x8c, b: 0x8d, a: 0},// BLACK
    Color {r: 0xc0, g: 0x39, b: 0x2b, a: 0},   // RED
    Color {r: 0x1c, g: 0xdc, b: 0x9a, a: 0},   // GREEN
    Color {r: 0xfd, g: 0xbc, b: 0x4b, a: 0}, // YELLOW
    Color {r: 0x3d, g: 0xae, b: 0xe9, a: 0},   // BLUE
    Color {r: 0x8e, g: 0x44, b: 0xad, a: 0}, // MAGENTA
    Color {r: 0x16, g: 0xa0, b: 0x85, a: 0}, // CYAN
    Color {r: 0xff, g: 0xff, b: 0xff, a: 0}, // WHITE
];

const BG_COL: Color = COL_ARR[0];
const BOLD_BG_COL: Color = Color {r: 0x00, g: 0x00, b: 0x00, a: 0};

const FG_COL: Color = COL_ARR[7];
const BOLD_FG_COL: Color = BOLD_COL_ARR[7];

impl TextArea {
    fn _set_cell(&mut self, i: usize, ch: char) -> bool {
        if i > self.width * self.height {
            return false;
        }
        self.cells[i].ch = ch;
        self.cells[i].fg_col = self.curr_fg_col;
        self.cells[i].bg_col = self.curr_bg_col;
        self.cells[i].bold = self.curr_is_bold;
        self.cells[i].dirty = true;
        true
    }

    pub fn set_cell(&mut self, x: u32, y: u32, ch: char) -> bool {
        if x >= self.width as u32 || y >= self.height as u32 {
            return false;
        }
        let i = (x+y*(self.width as u32)) as usize;
        return self._set_cell(i, ch);
    }

    fn scroll_up(&mut self, n: u32) {
        if n >= self.height as u32 {
            self.clear_screen(2);
        } else {
            let offset = ((n) as usize)*self.width;
            for i in 0..(self.width * self.height)-offset {
                self.cells[i] = Cell::from(self.cells[i+offset].clone());
            }

            for i in (self.width * self.height)-offset..(self.width * self.height) {
                self._set_cell(i, ' ');
            }
        }
    }

    fn scroll_down(&mut self, n: u32) {
        if n >= self.height as u32 {
            self.clear_screen(2);
        } else {
            let offset = ((n) as usize)*self.width;
            for i in ((self.width * self.height)-offset)..0 {
                self.cells[i] = Cell::from(self.cells[i+offset].clone());
            }

            for i in (self.width * self.height)-offset..(self.width * self.height) {
                self._set_cell(i, ' ');
            }
        }
    }

    fn _copy(&mut self, x: u32, y: u32, w: u32, h: u32, dx: u32, dy: u32) {
        let cloned_cells = self.cells.clone();

        for X in 0..w {
            for Y in 0..h {
                if X >= self.width as u32 || Y >= self.height as u32 {
                    continue;
                }
                let ix = x+X;
                let iy = y+Y;
                let dx = dx+X;
                let dy = dy+Y;
                if ix >= self.width as u32 || iy >= self.height as u32 {
                    self.set_cell(ix, iy, ' ');
                } else if dx >= self.width as u32 || dy >= self.height as u32 {
                    continue;
                } else {
                    let icell = ix+iy*(self.width as u32);
                    let dcell = dx+dy*(self.width as u32);
                    self.cells[dcell as usize] = Cell::from(cloned_cells[icell as usize].clone());
                }
            }
        }
    }

    pub fn print_char(&mut self, ch: char) {
        match ch {
            '\n' => {
                self.x = 0;
                self.y += 1;
            }
            '\r' => {
                self.x = 0;
            }
            '\x08' => {
                if self.x == 0 {
                    return;
                }
                self.x -= 1;
                self.set_cell(self.x, self.y, ' ');
            }
            _ => {
                if (ch as i32) < 0x20 {
                    println!("smol char: {:X}", ch as u8);
                    return;
                }

                self.set_cell(self.x, self.y, ch);
                self.x += 1;
                if self.x > self.width as u32 {
                    self.y += 1;
                    self.x = 0;
                }
            }
        }
        if self.y > self.bottom_margin {
            //self.scroll_up(self.y-self.bottom_margin);
            let diff = self.y-self.bottom_margin;
            let height = self.bottom_margin-self.top_margin;
            let to_y = self.top_margin;
            let from_y = self.top_margin+diff;
            self._copy(0, from_y, self.width as u32, height, 0, to_y);
            //self._copy(0, self.top_margin+(self.y-self.bottom_margin), self.width as u32, self.y-self.bottom_margin, 0, self.top_margin);
            println!("Setting self.y = {}", self.bottom_margin);
            self.y = self.bottom_margin;
            self.clear_line(2);
        }
        if self.y >= self.height as u32 {
            self.scroll_up(self.y-(self.height as u32)+1);
            self.y -= self.y-(self.height as u32)+1;
        }
    }

    fn clear_line(&mut self, n: u32) {
        match n {
            0 => {
                for x in self.x..(self.width as u32) {
                    self.set_cell(x, self.y, ' ');
                }
            }
            1 => {
                for x in 0..self.x {
                    self.set_cell(x, self.y, ' ');
                }
            }
            2 => {
                for x in 0..(self.width as u32) {
                    self.set_cell(x, self.y, ' ');
                }
            }
            _ => {}
        }
    }

    fn clear_screen(&mut self, n: u32) {
        let temp_y = self.y;
        match n {
            0 => {
                self.clear_line(n);
                if self.y < self.height as u32 {
                    self.y += 1;
                }
                for y in self.y..(self.height as u32) {
                    self.y = y;
                    self.clear_line(2);
                }
            }
            1 => {
                self.clear_line(n);
                if self.y > 0 {
                    self.y -= 1;
                }
                for y in 0..self.y {
                    self.y = y;
                    self.clear_line(2);
                }
            }
            2 => /* Clear screen */ {
                for y in 0..self.height {
                    self.y = y as u32;
                    self.clear_line(2);
                }
            }
            _ => {

            }
        }
        self.y = temp_y;
    }

    fn sgr(&mut self, mut n: u32, mut m: Vec<u32>) {
        let mut i = 0;
        match n {
            0 => {
                //self.set_cell_color(self.x,self.y,Color {r: 0, g: 0, b: 0, a: 0},Color {r: 0, g: 0, b: 0, a: 0});
                self.fg_is_default = true;
                if self.curr_is_bold {
                    self.curr_fg_col = BOLD_FG_COL;
                }else {
                    self.curr_fg_col = FG_COL;
                }
                self.curr_bg_col = BG_COL;
                self.curr_is_bold = false;
                self.inverse = false;
            }
            1 => {
                self.curr_is_bold = true;
            }
            7 => {
                self.inverse = true;
            }
            22 => {
                self.curr_is_bold = false;
            }
            39 => {
                // Reset fg col
                self.fg_is_default = true;
                if self.curr_is_bold {
                    self.curr_fg_col = BOLD_COL_ARR[7 as usize];
                } else {
                    self.curr_fg_col = COL_ARR[7 as usize];
                }
            }
            49 => {
                // Reset bg col
                self.curr_bg_col = COL_ARR[0 as usize];
            }
            mut a => {
                if a >= 90 && a <= 97 {
                    self.fg_is_default = false;
                    self.curr_fg_col = BOLD_COL_ARR[(a-90) as usize];
                }
                if a >= 100 && a <= 107 {
                    self.curr_bg_col = BOLD_COL_ARR[(a-100) as usize];
                }
                if a >= 30 && a <= 37 {
                    self.fg_is_default = false;
                    if self.curr_is_bold {
                        self.curr_fg_col = BOLD_COL_ARR[(a -30) as usize];
                    } else {
                        self.curr_fg_col = COL_ARR[(a -30) as usize];
                    }
                }
                if a >= 40 && a <= 47 {
                    self.curr_bg_col = COL_ARR[(a -40) as usize];
                }
                if a == 38 || a == 48 {
                    println!("COLOR MOMENT");
                    if i < m.len() {
                        match m[i] {
                            2 => {
                                println!("COLOR 2");
                                i+=1;
                                let r = m[i];
                                i+=1;
                                let g = m[i];
                                i+=1;
                                let b = m[i];
                            }
                            5 => {
                                println!("COLOR 5");
                                i+=1;
                                if m[i] <= 16 {

                                } else if m[i] >= 232 {
                                    let c = (0x8+(m[i]-232)*0xA) as u8;
                                    if a == 38 {
                                        self.fg_is_default = false;
                                        self.curr_fg_col = Color::RGB(c,c,c);
                                    } else {
                                        self.curr_bg_col = Color::RGB(c,c,c);
                                    }
                                    println!("RGB COLOR: {:x}, {:x}, {:x}", c, c, c)
                                } else {
                                    m[i] -= 16;
                                    let index_r = m[i] / 36;
                                    let index_g = (m[i] % 36) / 6;
                                    let index_b = m[i] % 6;

                                    let r = 55 + if index_r > 0 {index_r*40} else {0} as u8;
                                    let g = 55 + if index_g > 0 {index_g*40} else {0} as u8;
                                    let b = 55 + if index_b > 0 {index_b*40} else {0} as u8;

                                    if a == 38 {
                                        self.fg_is_default = false;
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
                            }
                            a => {
                                println!("UNKNOWN COLOR COMMAND: {}", a);
                            }
                        }
                    }
                }
            }
        }
        i+=1;
        if i < m.len() {
            m.drain(0..i);
            n = m[0].clone();
            m.remove(0);
            println!("Recursion: SGR n: {}, m: {:?}", n, m);
            self.sgr(n,m.clone());
        }
    }

    pub fn print_str(&mut self, s: &str, win: &mut sdl2::video::Window) {
        use crate::ansi_escaper::*;
        self.buf_str += s;

        let mut escaping = false;
        let buf_str = self.buf_str.clone();
        let mut ansi_string = String::new();
        let mut ansi_length = 0;

        /* Very very VERY weird code that takes the string and somehow finds all unicode chars and
         * makes them work i guess idk. Wrote when very tired
         */
        let mut output: Vec::<u8> = vec![];
        for char in buf_str.chars() {
            output.push(char as u8);
        }
        let utf8_str = CString::new(output.clone());

        /* Loop */
        for char in String::from(utf8_str.unwrap().as_c_str().to_str().unwrap()).chars() {
            if escaping {
                ansi_string.push(char);
                let res = crate::ansi_escaper::escape(ansi_string.clone());
                let res2 = res.0.clone();
                let reslen = res.1.clone();
                match res.0 {
                    AnsiType::Incomplete => {} // Ignore if Incomplete
                    _ => {
                        println!("Escape code: {:?}", ansi_string.clone());
                        /*for ch in ansi_string.clone().chars() {
                            if ch == '\x1b' {
                                print!("^");
                            } else {
                                print!("{}", ch);
                            }
                        }*/
                        //let res = crate::ansi_escaper::escape(ansi_string.clone());
                        println!("Escaped str: {}\nSize: {}\n", res2, reslen);
                    }
                }
                if res.1 > 0 {
                    match res.0 {
                        AnsiType::SS2 => {}
                        AnsiType::SS3 => {}
                        AnsiType::DCS => {}
                        AnsiType::CSI { kind } => {
                            match kind {
                                CSIType::CUU(n) => {self.y -= if self.y >= n { n } else { self.y }}
                                CSIType::CUD(n) => {
                                    self.y += n;
                                }
                                CSIType::CUF(n) => {self.x += n; if self.x > self.width as u32 {self.x = self.width as u32}}
                                CSIType::CUB(n) => {self.x -= if self.x >= n { n } else { self.x }}
                                CSIType::CNL(n) => {self.x = 0; self.y += n; if self.y > self.height as u32 {self.y = self.height as u32}}
                                CSIType::CPL(n) => {self.x = 0; self.y -= if self.y >= n { n } else { self.y }}
                                CSIType::CHA(n) => {self.x = n-1;}
                                CSIType::CVA(n) => {self.y = n-1;}
                                CSIType::CUP(n, m) => {
                                    self.y = if n > 0 {n-1} else {n};
                                    self.x = if m > 0 {m-1} else {m};
                                    println!("CUP: x={}, y={} ({})",self.x,self.y,self.bottom_margin);
                                }
                                CSIType::ED(n) => {
                                    self.clear_screen(n);
                                }
                                CSIType::EL(n) => {
                                    self.clear_line(n);
                                }
                                CSIType::SU(n) => {
                                    self.scroll_up(n);
                                }
                                CSIType::SD(n) => {
                                    self.scroll_down(n);
                                }
                                CSIType::IL(n) => {
                                    // Take into account the page margins
                                    let from_y = if self.y < self.top_margin {self.top_margin} else {
                                        if self.y > self.bottom_margin {
                                            self.bottom_margin
                                        } else {
                                            self.y
                                        }
                                    };
                                    let _to_y = self.y+n;
                                    let to_y = if _to_y < self.top_margin {self.top_margin} else {
                                        if _to_y >= self.bottom_margin {
                                            self.bottom_margin
                                        } else {
                                            _to_y
                                        }
                                    };
                                    let height = self.bottom_margin-self.top_margin;

                                    self._copy(0, from_y, self.width as u32, height, 0, to_y);
                                    self.clear_line(2);
                                }
                                CSIType::HVP(n, m) => {
                                    self.y = if n == 0 {0} else {n-1};
                                    self.x = if m == 0 {0} else {m-1};
                                }
                                CSIType::SGR(n, m) => {
                                    self.sgr(n,m);
                                }
                                CSIType::DECSLRM(n, m) => {
                                    // TODO: Implement left and right margins
                                }
                                CSIType::DECSTBM(n, m) => {
                                    self.top_margin = if n == 0 {0} else {n-1};
                                    self.bottom_margin = if m == 0 {0} else {m-1};
                                    println!("Top margin: {} ({}), Bottom margin: {} ({})", self.top_margin, n, self.bottom_margin, m);
                                    if self.bottom_margin <= self.top_margin {
                                        // Make sure the top margin is always less that the bottom
                                        self.bottom_margin = self.top_margin+1;
                                    }
                                    // Move to column 1, line 1 of the page
                                    self.x = 0;
                                    self.y = self.top_margin;
                                }
                                CSIType::Unknown(s) => {
                                    println!("UNKNOWN: {}", s);
                                }
                            }
                        }
                        AnsiType::ST => {}
                        AnsiType::OSC { kind } => {
                            match kind {
                                OSCType::WindowTitle(title) => {win.set_title(title.as_str());}
                                OSCType::Unknown(s) => {}
                            }
                        }
                        AnsiType::RIS => {}
                        AnsiType::SOS => {}
                        AnsiType::PM => {}
                        AnsiType::APC => {}
                        AnsiType::Incomplete => {}
                        AnsiType::Unknown(_) => {}
                    }
                    escaping = false;
                    ansi_string.clear();
                    ansi_length = res.1;
                    if self.y > self.bottom_margin {
                        //self.scroll_up(self.y-self.bottom_margin);
                        let diff = self.y-self.bottom_margin;
                        let height = self.bottom_margin-self.top_margin;
                        let to_y = self.top_margin;
                        let from_y = self.top_margin+diff;
                        self._copy(0, from_y, self.width as u32, height, 0, to_y);
                        //self._copy(0, self.top_margin+(self.y-self.bottom_margin), self.width as u32, self.y-self.bottom_margin, 0, self.top_margin);
                        println!("Setting self.y = {}", self.bottom_margin);
                        self.y = self.bottom_margin;
                        self.clear_line(2);
                    }
                    if self.x > self.width as u32 {
                        self.x = self.width as u32
                    }
                } /* if res.1 > 0 */ else {

                }
                continue;
            }
            match char {
                '\x1B' /* Escape */ => {
                    escaping = true;
                    ansi_string.push(char);
                }
                _ => {
                    self.print_char(char);
                }
            }
        } /* for chars in buf_str.chars() */

        self.buf_str.clear();
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, ren: &TermRenderer) {
        let tex_creator = canvas.texture_creator();
        if self.font_width == 0 || self.font_height == 0 {
            let char_surf = ren.font.render_char('i')
                .shaded(self.cells[0].fg_col, self.cells[0].bg_col)
                .unwrap();
            self.font_width = char_surf.width();
            self.font_height = char_surf.height();
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + y * self.width;
                if self.x == x as u32 && self.y == y as u32 {
                    self.cells[i].dirty = true;
                } else if !self.cells[i].dirty {
                    continue;
                }
                if self.cells[i].ch == '\0' {
                    self.cells[i].ch = ' ';
                }

                let mut fgc;
                let mut bgc;
                if self.inverse {
                    fgc = self.cells[i].bg_col;
                    bgc = self.cells[i].fg_col;
                } else {
                    fgc = self.cells[i].fg_col;
                    bgc = self.cells[i].bg_col;
                }

                if self.x == x as u32 && self.y == y as u32 {
                    let tmp_col = fgc;
                    fgc = bgc;
                    bgc = tmp_col;
                }

                let text_surf = if self.cells[i].bold {
                    ren.bold_font.render_char(self.cells[i].ch)
                        .shaded(fgc, bgc)
                        .unwrap()
                } else {
                    ren.font.render_char(self.cells[i].ch)
                        .shaded(fgc, bgc)
                        .unwrap()
                };

                let text_texture = text_surf.as_texture(&tex_creator).unwrap();
                let text_rect = Rect::new(0, 0, self.font_width, self.font_height);
                let real_rect = Rect::new((x * ((self.font_width) as usize)) as i32, (y * (self.font_height as usize)) as i32, self.font_width, self.font_height);
                canvas.copy(text_texture.borrow(), text_rect, real_rect).unwrap();
                if !(self.x == x as u32 && self.y == y as u32) {
                    self.cells[i].dirty = false;
                }
            }
        }
    }
}

pub fn create_ta(width: usize, height: usize, font_width: u32, font_height: u32) -> TextArea {
    let mut ta = TextArea {
        width,
        height,
        font_width,
        font_height,
        cells: std::vec::Vec::new(),

        x: 0,
        y: 0,
        buf_str: String::new(),

        curr_fg_col: FG_COL,
        curr_bg_col: BG_COL,
        fg_is_default: true,
        curr_is_bold: false,
        inverse: false,

        top_margin: 0,
        bottom_margin: (height - 1) as u32
    };

    for _ in 0..width*height {
        ta.cells.push(Cell{
            ch: ' ',
            fg_col: ta.curr_fg_col,
            bg_col: ta.curr_bg_col,
            bold: false,
            dirty: true
        });
    }

    println!("R: {:#X}, G: {:#X}, B: {:#X}", ta.cells[0].fg_col.r, ta.cells[0].fg_col.g, ta.cells[0].fg_col.b);

    ta
}