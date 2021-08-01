use std::fmt::{Display, Formatter, Error, Write};
use std::ops::Range;

#[derive(Clone)]
pub enum AnsiType {
    SS2, // Single Shift 2
    SS3, // Single Shift 3
    DCS, // Device Control String
    CSI {kind: CSIType}, // Control Sequence Introducer
    ST,  // String Terminator
    OSC {kind: OSCType}, // Operating System Command
    RIS, // Reset to Initial State

    // These three can be ignored (after parsing), as they are usually application specific
    SOS, // Start of String
    PM,  // Privacy Message
    APC, // Application Program Command

    Incomplete, // Ansi sequence is not complete / has errors

    Unknown(String),
}

impl From<char> for AnsiType {
    fn from(ch: char) -> Self {
        match ch {
            'N' =>  { AnsiType::SS2 }
            'O' =>  { AnsiType::SS3 }
            'P' =>  { AnsiType::DCS }
            '[' =>  { AnsiType::CSI{ kind: CSIType::Unknown(String::new()) } }
            '\\' => { AnsiType::ST }
            ']' =>  { AnsiType::OSC {kind: OSCType::Unknown(String::new()) } }
            'X' =>  { AnsiType::SOS }
            '*' =>  { AnsiType::PM }
            '_' =>  { AnsiType::APC }
            'c' =>  { AnsiType::RIS }
            _ => { AnsiType::Unknown(String::from(format!("Unknown ansi escape char: {}", ch))) }
        }
    }
}

impl AnsiType {
    pub fn finish(ch: char, t: AnsiType, args: Vec<String>) -> AnsiType {
        match t {
            AnsiType::SS2 => {AnsiType::ST}
            AnsiType::SS3 => {AnsiType::ST}
            AnsiType::DCS => {AnsiType::ST}
            AnsiType::CSI { .. } => {
                let csi = AnsiType::CSI { kind: CSIType::from(ch, args) };
                csi
            }
            AnsiType::ST => {AnsiType::ST}
            AnsiType::OSC { .. } => {AnsiType::OSC {kind: OSCType::from(ch, args)}}
            AnsiType::RIS => {AnsiType::ST}
            AnsiType::SOS => {AnsiType::ST}
            AnsiType::PM => {AnsiType::ST}
            AnsiType::APC => {AnsiType::ST}
            AnsiType::Incomplete => {AnsiType::ST}
            AnsiType::Unknown(s) => {AnsiType::Unknown(s)}
        }
    }

    pub fn valid_char_ranges(t: &AnsiType) -> (Range<u32>, Range<u32>) {
        let mut end_char_range = 1..0;
        (match t {
            AnsiType::SS2 => {1..0}
            AnsiType::SS3 => {1..0}
            AnsiType::DCS => {1..0}
            AnsiType::CSI { .. } => {end_char_range = 0x40..0x80; 0x20..0x40}
            AnsiType::ST => {1..0}
            AnsiType::OSC { .. } => {end_char_range = 0x7..0x8; 0x20..0x80}
            AnsiType::RIS => {1..0}
            AnsiType::SOS => {1..0}
            AnsiType::PM => {1..0}
            AnsiType::APC => {1..0}
            AnsiType::Incomplete => {1..0}
            AnsiType::Unknown(e_str) => {1..0}
        }, end_char_range)
    }
}

impl Display for AnsiType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let _ = match self {
            AnsiType::SS2 => {f.write_str("SS2")}
            AnsiType::SS3 => {f.write_str("SS3")}
            AnsiType::DCS => {f.write_str("DCS")}
            AnsiType::CSI { kind } => {
                let _ = match kind {
                    CSIType::CUU(n) => {
                        f.write_str(format!("CUU {{ n: {}", n).as_str())
                    }
                    CSIType::CUD(n) => {
                        f.write_str(format!("CUD {{ n: {}", n).as_str())
                    }
                    CSIType::CUF(n) => {f.write_str(format!("CUF {{ n: {}", n).as_str())}
                    CSIType::CUB(n) => {f.write_str(format!("CUB {{ n: {}", n).as_str())}
                    CSIType::CNL(n) => {f.write_str(format!("CNL {{ n: {}", n).as_str())}
                    CSIType::CPL(n) => {f.write_str(format!("CPL {{ n: {}", n).as_str())}
                    CSIType::CHA(n) => {f.write_str(format!("CHA {{ n: {}", n).as_str())}
                    CSIType::CVA(n) => {f.write_str(format!("CVA {{ n: {}", n).as_str())}
                    CSIType::CUP(n, m) => {f.write_str(format!("CUP {{ n: {}, m: {}", n, m).as_str())}
                    CSIType::ED(n) => {f.write_str(format!("ED {{ n: {}", n).as_str())}
                    CSIType::EL(n) => {f.write_str(format!("EL {{ n: {}", n).as_str())}
                    CSIType::SU(n) => {f.write_str(format!("SU {{ n: {}", n).as_str())}
                    CSIType::SD(n) => {f.write_str(format!("SD {{ n: {}", n).as_str())}
                    CSIType::IL(n) => {f.write_str(format!("IL {{ n: {}", n).as_str())}
                    CSIType::HVP(n, m) => {f.write_str(format!("HVP {{ n: {}, m: {}", n, m).as_str())}
                    CSIType::SGR(n, m) => {f.write_str(format!("SGR {{ n: {}, m: {:?}", n, m).as_str())}
                    CSIType::DECSTBM(n, m) => {f.write_str(format!("DECSTBM {{ n: {}, m: {:?}", n, m).as_str())}
                    CSIType::DECSLRM(n, m) => {f.write_str(format!("DECSLRM {{ n: {}, m: {:?}", n, m).as_str())}
                    CSIType::Unknown(s) => {f.write_str(format!("CSI {{ Unknown: {:?}", s).as_str())}
                };
                f.write_str(" }")
            } // End CSI

            AnsiType::ST => {f.write_str("ST")}
            AnsiType::OSC { kind } => {
                let _ = match kind {
                    OSCType::WindowTitle(s) => {f.write_str(format!("OSC {{ WindowTitle: {:?}", s).as_str())}
                    OSCType::Unknown(s) => {f.write_str(format!("OSC {{ Unknown: {:?}", s).as_str())}
                };
                f.write_str(" }")
            }
            AnsiType::RIS => {f.write_str("RIS")}
            AnsiType::SOS => {f.write_str("SOS")}
            AnsiType::PM => {f.write_str("PM")}
            AnsiType::APC => {f.write_str("APC")}
            AnsiType::Unknown(s) => {f.write_str(format!("Unknown: {:?}", s).as_str())}
            AnsiType::Incomplete => {f.write_str("Incomplete")}
        };
        Ok(())
    }
}

enum AnsiArgumentType {
    String(String),
    Int(u32),
    Unknown(String),
}

#[derive(Clone)]
pub enum OSCType {
    WindowTitle(String),
    Unknown(String),
}

#[derive(Clone)]
pub enum CSIType {
    // Cursor manipulation
    CUU(u32),
    CUD(u32),
    CUF(u32),
    CUB(u32),
    CNL(u32),
    CPL(u32),
    CHA(u32),
    CVA(u32),
    CUP(u32,u32),

    ED(u32),
    EL(u32),

    SU(u32),
    SD(u32),

    IL(u32),

    HVP(u32,u32),

    SGR(u32, Vec<u32>),

    DECSTBM(u32, u32),
    DECSLRM(u32, u32),

    Unknown(String),
}

impl OSCType {
    pub fn from(ch: char, args: Vec<String>) -> OSCType {
        match args[0].as_str() {
            "0" => /* BEL */ {
                OSCType::WindowTitle(args[1].clone())
            }
            _ => { OSCType::Unknown(String::from(format!("Unknown OSC command: {:?}", ch)))}
        }
    }
}

impl CSIType {
    pub fn from(ch: char, _args: Vec<String>) -> CSIType {
        let mut args = _args.clone();
        let mut private = false;
        if args[0].starts_with("?") {
            args[0].remove(0);
            private = true;
        }

        let first_arg_result = args[0].as_str().parse::<u32>();
        let n;
        let mut default = false;
        if first_arg_result.is_ok() {
            n = first_arg_result.unwrap();
        } else {
            n = 1;
            default = true;
        }

        let m;
        if args.len() > 1 {
            let m_res = args[1].as_str().parse::<u32>();
            if m_res.is_ok() {
                m = m_res.unwrap();
            } else {
                m = 1;
            }
        } else {
            m = 1;
        }

        if !private {
            match ch {
                'A' => { CSIType::CUU(n) }
                'B' => { CSIType::CUD(n) }
                'C' => { CSIType::CUF(n) }
                'D' => { CSIType::CUB(n) }
                'E' => { CSIType::CNL(n) }
                'F' => { CSIType::CPL(n) }
                'G' => { CSIType::CHA(n) }
                'd' => { CSIType::CVA(n) }
                'H' => { CSIType::CUP(n, m) }
                'J' => { CSIType::ED( if default {0} else {n} ) }
                'K' => { CSIType::EL( if default {0} else {n} ) }
                'L' => { CSIType::IL(n) }
                'S' => { CSIType::SU(n) }
                'T' => { CSIType::SD(n) }
                'f' => { CSIType::CUP(n, m) }
                'm' => {
                    let mut sgr_args = Vec::<u32>::new();
                    for i in 1..args.len() {
                        let res = args[i].as_str().parse::<u32>();
                        if res.is_ok() {
                            sgr_args.push(res.unwrap());
                        } else {
                            sgr_args.push(0);
                        }
                    }
                    CSIType::SGR(if default {0} else {n}, sgr_args)
                }
                'r' => { CSIType::DECSTBM(n, m) }
                's' => { CSIType::DECSLRM(n, m) }
                _ => { CSIType::Unknown(format!("Unknown CSI command: {}", ch)) }
            }
        } else {
            match n {
                _ => { CSIType::Unknown(format!("Unknown Private CSI command: {}", n)) }
            }
        }
    }
}

pub fn escape(s: String) -> (AnsiType,i32) {
    let byte_arr = s.as_bytes();
    if byte_arr.len() < 2 {
        return (AnsiType::Incomplete,0);
    }
    if byte_arr[0] != 0x1B /* Escape char */ {
        return (AnsiType::Unknown(String::from("First character not escape char")),1);
    }
    if byte_arr[1] == '>' as u8 {
        return (AnsiType::Unknown(String::from("I do not know how to handle this")),2);
    }
    if byte_arr.len() < 3 {
        return (AnsiType::Incomplete,0);
    }

    let t = AnsiType::from(char::from(byte_arr[1]));
    let mut end_char_range= 1..0;

    let char_ranges = AnsiType::valid_char_ranges(&t);
    let mut special = false;
    match t {
        AnsiType::CSI { .. } => {
            if byte_arr[2] != '?' as u8 {
                special = true;
            }
        }
        AnsiType::Unknown(e_str) => {return (AnsiType::Unknown(e_str),2)}
        _ => {}
    }

    let valid_char_ranges = char_ranges.0;
    end_char_range = char_ranges.1;

    let mut arguments: Vec<String> = Vec::new();
    let mut curr_arg = String::new();
    let mut i = 0;
    let mut escaping = false;
    let mut ansi_string = String::new();

    for char in s.chars() {
        if i < 2 { i += 1; continue; }
        i += 1;

        if char == '\x1b' || escaping {
            escaping = true;
            ansi_string.push(char);
            let res = crate::ansi_escaper::escape(ansi_string.clone());
            if res.1 > 0 {
                match res.0 {
                    AnsiType::ST => {return (AnsiType::finish('\x07', t, arguments),i);}
                    _ => {}
                }
                escaping = false;
            }
            continue;
        }

        if char == ';' {
            arguments.push(curr_arg.clone());
            curr_arg.clear();
            continue;
        }

        if valid_char_ranges.contains(&u32::from(char)) {
            curr_arg.push(char);
        } else if end_char_range.contains(&u32::from(char)) {
            arguments.push(curr_arg.clone());
            // Get CSI Type
            return (AnsiType::finish(char, t, arguments), i);

        } else {
            return (AnsiType::Unknown(format!("Illegal character {:?} found in escape sequence", char)),i);
        }
    }

    (AnsiType::Incomplete,0)
}