mod serial;

pub fn init() {
    #[cfg(target_arch = "x86_64")]
    serial::init();
}

pub fn write_str(s: &str) {
    #[cfg(target_arch = "x86_64")]
    serial::write_str(s);
}

pub fn write_fmt(args: core::fmt::Arguments) {
    use core::fmt::Write;

    struct Writer;
    impl Write for Writer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            crate::console::write_str(s);
            Ok(())
        }
    }

    let mut w = Writer;
    let _ = w.write_fmt(args);
}
