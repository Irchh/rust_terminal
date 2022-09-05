use rust_ansi::term::TermInterface;

pub struct Terminal {
    x: u32
}

impl TermInterface for Terminal {
    fn write(&self, s: String) {
        todo!()
    }

    fn goto_x(&self, x: usize) {
        todo!()
    }

    fn goto_y(&self, y: usize) {
        todo!()
    }

    fn move_x(&self, x: isize) {
        todo!()
    }

    fn move_y(&self, y: isize) {
        todo!()
    }

    fn erase_in_display(&self, n: usize) {
        todo!()
    }

    fn erase_in_line(&self, n: usize) {
        todo!()
    }

    fn scroll_up(&self, n: usize) {
        todo!()
    }

    fn scroll_down(&self, n: usize) {
        todo!()
    }

    fn il(&self, n: usize) {
        todo!()
    }

    fn select_graphics_rendition(&self, n: usize, m: Vec<usize>) {
        todo!()
    }

    fn decstbm(&self, top: usize, bot: usize) {
        todo!()
    }

    fn decslrm(&self, left: usize, right: usize) {
        todo!()
    }

    fn device_status_report(&self) -> (usize, usize) {
        todo!()
    }

    fn unknown_csi(&self, s: String) {
        todo!()
    }

    fn set_title(&self, title: String) {
        todo!()
    }

    fn unknown_osc(&self, s: String) {
        todo!()
    }

    fn unknown(&self, s: String) {
        todo!()
    }
}