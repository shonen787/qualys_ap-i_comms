use std::io::{stdin,stdout,Write};

pub fn user_input(s: &mut String) -> String{
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();

    }
    s.to_string()
}
