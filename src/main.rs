mod lexer;

fn main () {
    let input = "g g c#";
    let tokens = lexer::tokenize(input);
    println!("Tokens: {:?}", tokens);
}