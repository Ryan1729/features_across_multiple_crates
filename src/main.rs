extern crate common;
extern crate middle;

fn main() {
    common::check!();

    middle::run();

    println!("done");
}
