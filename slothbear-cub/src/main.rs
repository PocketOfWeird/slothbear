use std::process::Command;
use std::io::{self, Write};

fn main() {
    let mut say_hello = Command::new("cmd");
    say_hello.arg("/c").arg("echo hello");

    println!("Starting Command");

    let hello_1 = say_hello.output().expect("failed to execute cmd");
    let hello_2 = say_hello.output().expect("failed to execute cmd");

    println!("hello_1 status: {}", hello_1.status);
    println!("hello_2 status: {}", hello_2.status);
    io::stdout().write_all(&hello_1.stdout).unwrap();
    io::stderr().write_all(&hello_1.stderr).unwrap();
    io::stdout().write_all(&hello_2.stdout).unwrap();
    io::stderr().write_all(&hello_2.stderr).unwrap();

}
