
use std::env;

fn main() {
    loop {
        // Skip the first argument (program) and collect the rest of the arguments in vector
        let argument: Vec<String> = env::args().skip(1).collect();
        let cmd: &String = &argument[0];
        let cmd: String = cmd.to_lowercase();

        // Joins all the vector elements and separates with a space
        let input = argument[1..].join(" ");

        if cmd == "upper" {
            println!("{}", input.to_uppercase());
            break
        } else if cmd == "lower" {
            println!("{}", input.to_lowercase());
            break
        } else {
            continue
        }
    }
 
}
