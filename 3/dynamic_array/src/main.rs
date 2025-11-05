use std::io;

fn main() {
    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    // Okay this does not seem possible but good exercise

    const A: usize = a.trim().parse().expect("Please type a number!");

    let arr= [1; A];

    println!("{}", arr[A - 1]);
}
