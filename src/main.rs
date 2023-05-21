// use std::io;

// cargo add rand@0.8.5
use rand::Rng;
use std::cmp::Ordering;
// yoo: cargo doc --open

fn main() {
    println!("Sugondese");
    // this range operator... magic
    // oh no this changed type, based on a comparison to an explicitly typed variable later, the TS in PTSD stands for TypeScr*pt
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");
        // so what am I doing? Is this basically a malloc for a string? but it's growable
        let mut guess: String = String::new();
        let result = std::io::stdin()
            // the variable literally switches a stack, wow
            // oh it actually appends to the string
            .read_line(&mut guess);

        // but it is JavaScript because I can shadow variable within a scope.
        // This will be very very confusing in robust codebases, where every variable is name foo69, because naming things is harder.
        let guess: u32 = match guess.trim().parse/*::<u32>*/() {
            Ok(num) => num,
            // match is smart to figure out I can return a value OR control flow inline
            Err(_) => continue,
        };

        println!("The secret number is: {secret_number}");
        // oof, this no parens default will be hard to get used to
        if result.is_ok() {
            // this makes me question why this isn't the default string syntax in every language. For C# it's opt-in with $""
            // I can't put expressions in here, bummer
            let val = result.unwrap();
            println!("You guess: {guess}. Bytes read: {val}");
            // oh no the string read retains the line feed, haha
            match guess.cmp(&secret_number) {
                // wait the cmp() returns an enum?
                Ordering::Less => println!("PP small"),
                Ordering::Greater => println!("Big PP energy"), // no LSP if lowercase is typed, :'(
                Ordering::Equal => {
                    println!("Smack bang in the middle");
                    break;
                }
            }
        } else {
            // oh no, I'm sure this syntax will never cause problems when refactoring/adding features
            eprintln!("{}", result.unwrap_err());
            eprintln!("You thunk, but you thunk wrong!");
        }
    }
    let x: i8 = 5;
    let y: i32 = 10;
    println!(
        "x = {x} and y + 2 = {} and x + y = {}",
        y + 2,
        i32::from(x) + y
    );
}
