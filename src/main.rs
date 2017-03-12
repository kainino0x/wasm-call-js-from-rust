#![feature(link_args)]

extern crate libc;

mod externs;

fn main() {
    externs::alert("Hello, alert!");
    externs::eval("console.log('Hello, eval!')");
    println!("Hello, console!");
}
