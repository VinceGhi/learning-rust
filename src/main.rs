use std::io;
fn main() {
    let mut user_input: String = String::new();
    println!("Enter some random shit:");
    io::stdin().read_line(&mut user_input).expect("Failed to read line! Idiot...");
    print!("Good job.. you indeed entered some random shit: {}", user_input);
}
