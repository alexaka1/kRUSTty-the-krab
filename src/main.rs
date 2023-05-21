// use std::io;

fn main() {
    println!("Sugondese");

    println!("Guess the number!");
    println!("Please input your guess.");
    // so what am I doing? Is this basically a malloc for a string? but it's growable
    let mut guess: String = String::new();
    std::io::stdin()
        // the variable literally switches a stack, wow
        .read_line(&mut guess)
        .expect("Failed to read line");
    // this makes me question why this isn't the default string syntax in every language. For C# it's opt-in with $""
    println!("You guess: {guess}");
}
