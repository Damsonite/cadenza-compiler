mod evaluator;

fn main () {
    let input = "d#ce";
    println!("{}", evaluator::notes_to_value(input));
}