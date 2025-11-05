fn main() {
    println!("This is 20C in Farenheit : {}", celcius_to_farenheit(20.0));
    println!("This is -40F in C : {}", farenheit_to_celcius(-40.0));
}

fn celcius_to_farenheit(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}

fn farenheit_to_celcius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}
