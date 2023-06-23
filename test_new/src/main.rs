use std::io;

fn main() {
    println!("Hello, world!");
    println!("Input New Char");
    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read the input");
    print!("you guessed {}",guess );

}
