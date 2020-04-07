use std::collections::VecDeque;
use std::fs::File;

fn main() {
    let mut queue_file = File::create("slothbear.queue").expect("Unable to open or create the slothbear.queue database file");
    let mut queue_file_contents = vec![];
    queue_file .read(&mut queue_file_contents).expect("Unable to read the slothbear.queue database contents");

    println!("hello");
}
