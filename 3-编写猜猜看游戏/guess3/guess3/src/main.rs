use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    //let t;

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed: {}", guess);
    
    //rust错误处理 result方式
    //只是知道可以match一个表达式,然后写错误处理,具体还不太懂

    let guess : u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(num) => return,
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    println!("Guess the number!");


}