use std::io;

fn main() {
    println!("Enter the password");
    let pw = "doubleK";

    let mut input_buffer = String::new();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read line");
    let parsed = input_buffer.trim();
    if parsed == pw {
        println!("That is correct!");
    } else {
        println!("Wrong password!");
    }
}
