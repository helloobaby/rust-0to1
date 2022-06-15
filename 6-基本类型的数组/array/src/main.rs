#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    println!("{:?}", primes);
    println!("{:?}", doubles);

    let numbers = [0;15];
    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;
    let mut numbers = [DEFAULT; 15];
    println!("{:?}", numbers);
    println!("{:?}", numbers[3]);

    numbers[3] = 5;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number);
    }
}