use core::{arch::asm, fmt::Write};

const SYSCALL_WRITE: usize = 64;
const STDOUT: usize = 1;

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\r\n") $(, $($arg)+)?));
    }
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        syscall(
            SYSCALL_WRITE,
            [STDOUT, s.as_bytes().as_ptr() as usize, s.as_bytes().len()],
        );
        Ok(())
    }
}

pub fn print(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    // push_trace(TRACE_SYSCALL_ENTER + id);
    unsafe {
        asm!("ecall", inout("a0") args[0] => ret, in("a1") args[1],
             in("a2") args[2], in("a7") id)
    }
    // push_trace(TRACE_SYSCALL_EXIT + id);
    ret
}
