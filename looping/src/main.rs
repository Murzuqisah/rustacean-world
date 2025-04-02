use std::io;

fn main() {
    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    let answer = "The letter e";
    let mut tries = 1;

    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        if answer == input.trim() {
            println!("Number of trials: {}", tries);
            break;
        } else {
            println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        }
        tries += 1;
    }

}
