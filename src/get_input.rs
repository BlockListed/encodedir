fn getinput(p: &str) -> Option<String> {
    use std::io;
    use std::io::prelude::*;
    let mut input: String = String::new();

    print!("{}", p);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string!");

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    return Some(input);
}
