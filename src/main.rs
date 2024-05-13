mod parser;
mod object;

fn main() {
    parser::parser("resources/42.obj").unwrap()
}
