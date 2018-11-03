#[macro_export]
macro_rules! check {
    () => {{
        if cfg!(feature = "invariant-checking") {
            println!("invariant-checking {}:{}", file!(), line!());
        }
        if cfg!(feature = "logging") {
            println!("logging {}:{}", file!(), line!());
        }
    }};
}

pub fn run() {
    check!()
}
