mod renderer;
mod text_area;
mod tty;
mod term;

extern crate sdl2;
extern crate nix;
extern crate libc;
extern crate rust_ansi;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rust_ansi::ansi_escaper;
use rust_ansi::term::Term;
use crate::renderer::TermRenderer;
use crate::term::Terminal;
use crate::text_area::TextArea;

fn main() {
    main_new();
}

fn main_old() {
    let con = tty::ForkPTY::new(80, 24);
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let ren = TermRenderer::new(&sdl_context, &ttf_context, 80, 24);
    let char_surf = ren.font.render_char('i')
        .shaded(Color{r:0,g:0,b:0,a:0}, Color{r:0,g:0,b:0,a:0})
        .unwrap();

    let window = ren.video_subsystem.window("Rust SDL2 Window!", char_surf.width()*80, char_surf.height()*24)
        .opengl()
        .build()
        .unwrap();

    println!("Is text input active?: {}", ren.video_subsystem.text_input().is_active());

    let mut ta = TextArea::new((window.size().0 / char_surf.width()) as usize, (window.size().1 / char_surf.height()) as usize,
                                      char_surf.width(), char_surf.height());

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    std::thread::sleep(Duration::new(0, 10000000)); // Allow sdl to init before drawing anything

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = ren.sdl_context.event_pump().unwrap();

    'running: loop {
        let conres = con.read();
        match conres {
            Ok(_) => {}
            Err(_) => {
                println!("Child process exited!");
                break 'running;
            }
        }
        let conres = conres.unwrap();

        if conres.1 > 0 {
            let mut res_str = String::new();
            for i in 0..conres.1 {
                res_str.push(char::from(conres.0[i]))
            }
            println!("Read: {:?}", res_str);
            ta.print_str(res_str.as_str(), &mut canvas.window_mut());
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    break 'running
                },
                Event::TextInput { text, .. } => {
                    con.write(text);
                }
                Event::KeyDown { keycode, .. } => {
                    if keycode.clone().is_none() {
                        continue;
                    }
                    match keycode.clone().unwrap() {
                        Keycode::Backspace => {con.write(String::from('\x08'));}
                        Keycode::Tab => {con.write(String::from('\t'));}
                        Keycode::Return => {con.write(String::from('\n'));}
                        Keycode::Escape => {con.write(String::from('\x1b'));}
                        Keycode::Space => {}
                        Keycode::Exclaim => {}
                        Keycode::Quotedbl => {}
                        Keycode::Hash => {}
                        Keycode::Dollar => {}
                        Keycode::Percent => {}
                        Keycode::Ampersand => {}
                        Keycode::Quote => {}
                        Keycode::LeftParen => {}
                        Keycode::RightParen => {}
                        Keycode::Asterisk => {}
                        Keycode::Plus => {}
                        Keycode::Comma => {}
                        Keycode::Minus => {}
                        Keycode::Period => {}
                        Keycode::Slash => {}
                        Keycode::Num0 => {}
                        Keycode::Num1 => {}
                        Keycode::Num2 => {}
                        Keycode::Num3 => {}
                        Keycode::Num4 => {}
                        Keycode::Num5 => {}
                        Keycode::Num6 => {}
                        Keycode::Num7 => {}
                        Keycode::Num8 => {}
                        Keycode::Num9 => {}
                        Keycode::Colon => {}
                        Keycode::Semicolon => {}
                        Keycode::Less => {}
                        Keycode::Equals => {}
                        Keycode::Greater => {}
                        Keycode::Question => {}
                        Keycode::At => {}
                        Keycode::LeftBracket => {}
                        Keycode::Backslash => {}
                        Keycode::RightBracket => {}
                        Keycode::Caret => {}
                        Keycode::Underscore => {}
                        Keycode::Backquote => {}
                        Keycode::Delete => {con.write(String::from('\x7F'));}
                        Keycode::CapsLock => {}
                        Keycode::F1 => {}
                        Keycode::F2 => {}
                        Keycode::F3 => {}
                        Keycode::F4 => {}
                        Keycode::F5 => {}
                        Keycode::F6 => {}
                        Keycode::F7 => {}
                        Keycode::F8 => {}
                        Keycode::F9 => {}
                        Keycode::F10 => {}
                        Keycode::F11 => {}
                        Keycode::F12 => {}
                        Keycode::PrintScreen => {}
                        Keycode::ScrollLock => {}
                        Keycode::Pause => {}
                        Keycode::Insert => {}
                        Keycode::Home => {}
                        Keycode::PageUp => {}
                        Keycode::End => {}
                        Keycode::PageDown => {}
                        Keycode::Right => {con.write(String::from("\x1B[C"));}
                        Keycode::Left => {con.write(String::from("\x1B[D"));}
                        Keycode::Down => {con.write(String::from("\x1B[B"));}
                        Keycode::Up => {con.write(String::from("\x1B[A"));}
                        Keycode::NumLockClear => {}
                        Keycode::KpDivide => {}
                        Keycode::KpMultiply => {}
                        Keycode::KpMinus => {}
                        Keycode::KpPlus => {}
                        Keycode::KpEnter => {}
                        Keycode::Kp1 => {}
                        Keycode::Kp2 => {}
                        Keycode::Kp3 => {}
                        Keycode::Kp4 => {}
                        Keycode::Kp5 => {}
                        Keycode::Kp6 => {}
                        Keycode::Kp7 => {}
                        Keycode::Kp8 => {}
                        Keycode::Kp9 => {}
                        Keycode::Kp0 => {}
                        Keycode::KpPeriod => {}
                        Keycode::Application => {}
                        Keycode::Power => {}
                        Keycode::KpEquals => {}
                        Keycode::F13 => {}
                        Keycode::F14 => {}
                        Keycode::F15 => {}
                        Keycode::F16 => {}
                        Keycode::F17 => {}
                        Keycode::F18 => {}
                        Keycode::F19 => {}
                        Keycode::F20 => {}
                        Keycode::F21 => {}
                        Keycode::F22 => {}
                        Keycode::F23 => {}
                        Keycode::F24 => {}
                        Keycode::Execute => {}
                        Keycode::Help => {}
                        Keycode::Menu => {}
                        Keycode::Select => {}
                        Keycode::Stop => {}
                        Keycode::Again => {}
                        Keycode::Undo => {}
                        Keycode::Cut => {}
                        Keycode::Copy => {}
                        Keycode::Paste => {}
                        Keycode::Find => {}
                        Keycode::Mute => {}
                        Keycode::VolumeUp => {}
                        Keycode::VolumeDown => {}
                        Keycode::KpComma => {}
                        Keycode::KpEqualsAS400 => {}
                        Keycode::AltErase => {}
                        Keycode::Sysreq => {}
                        Keycode::Cancel => {}
                        Keycode::Clear => {}
                        Keycode::Prior => {}
                        Keycode::Return2 => {}
                        Keycode::Separator => {}
                        Keycode::Out => {}
                        Keycode::Oper => {}
                        Keycode::ClearAgain => {}
                        Keycode::CrSel => {}
                        Keycode::ExSel => {}
                        Keycode::Kp00 => {}
                        Keycode::Kp000 => {}
                        Keycode::ThousandsSeparator => {}
                        Keycode::DecimalSeparator => {}
                        Keycode::CurrencyUnit => {}
                        Keycode::CurrencySubUnit => {}
                        Keycode::KpLeftParen => {}
                        Keycode::KpRightParen => {}
                        Keycode::KpLeftBrace => {}
                        Keycode::KpRightBrace => {}
                        Keycode::KpTab => {}
                        Keycode::KpBackspace => {}
                        Keycode::KpA => {}
                        Keycode::KpB => {}
                        Keycode::KpC => {}
                        Keycode::KpD => {}
                        Keycode::KpE => {}
                        Keycode::KpF => {}
                        Keycode::KpXor => {}
                        Keycode::KpPower => {}
                        Keycode::KpPercent => {}
                        Keycode::KpLess => {}
                        Keycode::KpGreater => {}
                        Keycode::KpAmpersand => {}
                        Keycode::KpDblAmpersand => {}
                        Keycode::KpVerticalBar => {}
                        Keycode::KpDblVerticalBar => {}
                        Keycode::KpColon => {}
                        Keycode::KpHash => {}
                        Keycode::KpSpace => {}
                        Keycode::KpAt => {}
                        Keycode::KpExclam => {}
                        Keycode::KpMemStore => {}
                        Keycode::KpMemRecall => {}
                        Keycode::KpMemClear => {}
                        Keycode::KpMemAdd => {}
                        Keycode::KpMemSubtract => {}
                        Keycode::KpMemMultiply => {}
                        Keycode::KpMemDivide => {}
                        Keycode::KpPlusMinus => {}
                        Keycode::KpClear => {}
                        Keycode::KpClearEntry => {}
                        Keycode::KpBinary => {}
                        Keycode::KpOctal => {}
                        Keycode::KpDecimal => {}
                        Keycode::KpHexadecimal => {}
                        Keycode::LCtrl => {}
                        Keycode::LShift => {}
                        Keycode::LAlt => {}
                        Keycode::LGui => {}
                        Keycode::RCtrl => {}
                        Keycode::RShift => {}
                        Keycode::RAlt => {}
                        Keycode::RGui => {}
                        Keycode::Mode => {}
                        Keycode::AudioNext => {}
                        Keycode::AudioPrev => {}
                        Keycode::AudioStop => {}
                        Keycode::AudioPlay => {}
                        Keycode::AudioMute => {}
                        Keycode::MediaSelect => {}
                        Keycode::Www => {}
                        Keycode::Mail => {}
                        Keycode::Calculator => {}
                        Keycode::Computer => {}
                        Keycode::AcSearch => {}
                        Keycode::AcHome => {}
                        Keycode::AcBack => {}
                        Keycode::AcForward => {}
                        Keycode::AcStop => {}
                        Keycode::AcRefresh => {}
                        Keycode::AcBookmarks => {}
                        Keycode::BrightnessDown => {}
                        Keycode::BrightnessUp => {}
                        Keycode::DisplaySwitch => {}
                        Keycode::KbdIllumToggle => {}
                        Keycode::KbdIllumDown => {}
                        Keycode::KbdIllumUp => {}
                        Keycode::Eject => {}
                        Keycode::Sleep => {}
                        kc => {
                            println!("Unhandled key pressed: {kc}")
                        }
                    }

                    //ta.print_char(ch.to_ascii_lowercase());
                },
                _e => {
                    //println!("{:?}", _e)
                }
            }
        }
        ta.render(&mut canvas, &ren);

        for sc in event_pump.keyboard_state().pressed_scancodes() {
            match sc {
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn main_new() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut renderer = TermRenderer::new(&sdl_context, &ttf_context, 80, 24);

    let window = renderer.video_subsystem.window("Terminal", renderer.font_width*80, renderer.font_height*24)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    std::thread::sleep(Duration::new(0, 10000000)); // Allow sdl to init before drawing anything

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = renderer.sdl_context.event_pump().unwrap();

    //let con = tty::ForkPTY::new(80, 24);
    let mut terminal = Term::new(Box::new(Terminal::new(80, 24)));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                }
                _ => {}
            }
        }
        terminal.write("\x1B[0mGHe\x1B[0m");
        renderer.render(terminal.framebuffer(), &mut canvas);
        terminal.completed_render();
        canvas.present();
        //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}