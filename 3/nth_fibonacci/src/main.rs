const FIRST_FIBONACCI: u32 = 1;
const SECOND_FIBONACCI: u32 = 2;

fn main() {
    println!("5th Fibonacci number: {}", nth_fibonacci(5));
}

fn nth_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return FIRST_FIBONACCI;
    }

    if n == 2 {
        return SECOND_FIBONACCI;
    }

    nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}
