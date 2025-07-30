use parser::parse_expr;

fn main() {
    let input = "_ > 0 ? 1 : -1";
    match parse_expr(input) {
        Ok(expr) => println!("{:#?}", expr),
        Err(e) => eprintln!("Parse error: {:?}", e),
    }
}
