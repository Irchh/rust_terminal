use nix::pty::ForkptyResult;
use nix::unistd::{ForkResult, sleep, read, write};
use std::ffi::CString;
use nix::errno::Errno;
use nix::Error;
use nix::sys::uio::pwrite;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};

pub struct ForkPTY {
    fork_res: Option<ForkptyResult>,
}

impl ForkPTY {
    pub fn new(width: u16, height: u16) -> ForkPTY {
        let mut res = ForkPTY {
            fork_res: None,
        };
        res.open(width, height);

        res
    }
    pub fn open(&mut self, width: u16, height: u16) {
        let ws = libc::winsize {
            ws_row: height,
            ws_col: width,
            ws_xpixel: 0,
            ws_ypixel: 0
        };
        let response = unsafe {nix::pty::forkpty(&ws, None).unwrap()};
        self.fork_res = Option::from(response);

        match response.fork_result {
            ForkResult::Parent { child } => {
                unsafe {libc::fcntl(response.master, libc::F_SETFL, libc::fcntl(response.master, libc::F_GETFL) | libc::O_NONBLOCK);}
            }
            ForkResult::Child => {
                //std::env::set_var("LC_ALL", "POSIX");
                /*
                let executable = CString::new("/bin/bash".as_bytes()).unwrap();
                let argv: [CString; 3] = [executable.clone(), CString::new("-l").unwrap(), CString::new("-i").unwrap()];// */
                //*
                let executable = CString::new("/usr/bin/env".as_bytes()).unwrap();
                let argv: [CString; 3] = [executable.clone(), CString::new("TERM=xterm-256color").unwrap(), CString::new("/bin/fish").unwrap()]; // */
                /*
                let executable = CString::new("/usr/bin/watch".as_bytes()).unwrap();
                let argv: [CString; 3] = [executable.clone(), CString::new("-n1").unwrap(), CString::new("free -h").unwrap()]; // */
                /*
                let executable = CString::new("/usr/bin/htop".as_bytes()).unwrap();
                let argv: [CString; 1] = [executable.clone()]; // */
                /*
                let executable = CString::new("/usr/bin/env".as_bytes()).unwrap();
                let argv: [CString; 3] = [executable.clone(), CString::new("TERM=xterm-256color").unwrap(), CString::new("top").unwrap()]; // */
                /*
                let executable = CString::new("/usr/bin/echo".as_bytes()).unwrap();
                let argv: [CString; 2] = [executable.clone(), CString::new("▽æ▽").unwrap()]; // */
                /*
                let executable = CString::new("/usr/bin/vim".as_bytes()).unwrap();
                let argv: [CString; 1] = [executable.clone()]; // */
                /*
                let executable = CString::new("/usr/bin/env".as_bytes()).unwrap();
                let argv: [CString; 3] = [executable.clone(), CString::new("TERM=xterm-256color").unwrap(), CString::new("vim").unwrap()]; // */

                let res = nix::unistd::execv(executable.as_c_str(), &argv);
            }
        };
    } /* pub fn open */

    pub fn read(&self) -> Result<([u8; 4096], usize), ()> {
        let mut t: [u8; 4096] = [0; 4096];
        if self.fork_res.is_none() {
            return Ok((t, 0))
        }
        match self.fork_res.unwrap().fork_result {
            ForkResult::Parent { child } => {
                let result = waitpid(child, Some(WaitPidFlag::WNOHANG));
                match result {
                    Ok(status) => {
                        match status {
                            WaitStatus::Signaled(_, _, _) => todo!(),
                            WaitStatus::Stopped(_, _) => todo!(),
                            WaitStatus::PtraceEvent(_, _, _) => todo!(),
                            WaitStatus::PtraceSyscall(_) => todo!(),
                            WaitStatus::Continued(_) => todo!(),
                            WaitStatus::StillAlive => {}
                            _ => return Err(()),
                        }
                    }
                    Err(_) => {
                        return Err(())
                    }
                }
            }
            ForkResult::Child => {}
        }
        let read_len = read(self.fork_res.unwrap().master, &mut t);
        match read_len {
            Ok(_) => Ok((t,read_len.unwrap())),
            Err(_) => Ok((t, 0)),
        }
    }

    pub fn write(&self, s: String) -> usize {
        write(self.fork_res.unwrap().master, s.as_bytes()).unwrap()
    }
}