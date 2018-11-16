#[cfg(any(target_os = "linux", target_os = "macos"))]
mod imp {
    use std::mem;

    use libc;

    use super::Shell;

    pub fn stderr_width() -> Option<usize> {
        unsafe {
            let mut winsize: libc::winsize = mem::zeroed();
            if libc::ioctl(libc::STDERR_FILENO, libc::TIOCGWINSZ.into(), &mut winsize) < 0 {
                return None;
            }
            if winsize.ws_col > 0 {
                Some(winsize.ws_col as usize)
            } else {
                None
            }
        }
    }
}

#[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
mod imp {
    pub fn stderr_width() -> Option<usize> {
        None
    }
}

#[cfg(windows)]
mod imp {
    extern crate winapi;

    use std::{cmp, mem, ptr};
    use self::winapi::um::fileapi::*;
    use self::winapi::um::handleapi::*;
    use self::winapi::um::processenv::*;
    use self::winapi::um::winbase::*;
    use self::winapi::um::wincon::*;
    use self::winapi::um::winnt::*;

    pub(super) use super::default_err_erase_line as err_erase_line;

    pub fn stderr_width() -> Option<usize> {
        unsafe {
            let stdout = GetStdHandle(STD_ERROR_HANDLE);
            let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
            if GetConsoleScreenBufferInfo(stdout, &mut csbi) != 0 {
                return Some((csbi.srWindow.Right - csbi.srWindow.Left) as usize)
            }

            // On mintty/msys/cygwin based terminals, the above fails with
            // INVALID_HANDLE_VALUE. Use an alternate method which works
            // in that case as well.
            let h = CreateFileA("CONOUT$\0".as_ptr() as *const CHAR,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                ptr::null_mut(),
                OPEN_EXISTING,
                0,
                ptr::null_mut()
            );
            if h == INVALID_HANDLE_VALUE {
                return None;
            }

            let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
            let rc = GetConsoleScreenBufferInfo(h, &mut csbi);
            CloseHandle(h);
            if rc != 0 {
                let width = (csbi.srWindow.Right - csbi.srWindow.Left) as usize;
                // Unfortunately cygwin/mintty does not set the size of the
                // backing console to match the actual window size. This
                // always reports a size of 80 or 120 (not sure what
                // determines that). Use a conservative max of 60 which should
                // work in most circumstances. ConEmu does some magic to
                // resize the console correctly, but there's no reasonable way
                // to detect which kind of terminal we are running in, or if
                // GetConsoleScreenBufferInfo returns accurate information.
                return Some(cmp::min(60, width));
            }
            return None;
        }
    }
}

pub use imp::stderr_width;

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_stderr_width() {
        assert_eq!(super::stderr_width(), Some(42));
    }

    #[test]
    #[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
    fn test_stderr_width() {
        assert_eq!(super::stderr_width(), None);
    }

    #[test]
    #[cfg(windows)]
    fn test_stderr_width() {
        assert_eq!(super::stderr_width(), Some(42));
    }
}
