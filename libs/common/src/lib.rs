#[cfg(feature = "invariant-checking")]
#[macro_export]
macro_rules! invariant_check {
    () => {
        println!("invariant-checking {}:{}", file!(), line!());
    };
}

#[cfg(not(feature = "invariant-checking"))]
#[macro_export]
macro_rules! invariant_check {
    () => {};
}

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! logging_check {
    () => {
        println!("logging {}:{}", file!(), line!());
    };
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! logging_check {
    () => {};
}

//In practice we would use these macros separately.
//This part is just so we don't change the existing code.
#[macro_export]
macro_rules! check {
    () => {
        common::invariant_check!();
        common::logging_check!();
    };
    (common) => {{
        invariant_check!();
        logging_check!();
    }};
}

pub fn run() {
    check!(common)
}
