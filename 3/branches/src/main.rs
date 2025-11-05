fn main() {
    let num: i32 = if false { 8 } else { 6 };

    if num % 2 == 0 {
        println!("{} is even", num);
    } else if num % 3 == 0 {
        println!("{} is divisible by 3", num);
    } else if num % 5 == 0 {
        println!("{} is divisible by 5", num);
    }

    let mut i: i32 = 0;

    let out = 'counter: loop {
        i += 1;
        let mut j = i;

        loop {
            j += 1;
            if j == 23 && i == 4 {
                break 'counter i + j;
            }
            if j == 23 {
                break;
            }
        }
    };
    println!("{}", out);
}
