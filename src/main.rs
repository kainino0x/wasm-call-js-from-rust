#![feature(link_args)]

extern crate libc;

mod externs;

fn main() {
    // Calls JS alert
    externs::alert("Hello, alert!");
    // Calls JS eval
    externs::eval("console.log('Hello, eval!')");
    // Redirects to JS console.log
    println!("Hello, console!");
}
