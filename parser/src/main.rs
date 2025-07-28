use parser::parse_expr;

fn main() {
    let input = "x > 0 ? y : z";
    match parse_expr(input) {
        Ok(expr) => println!("{:#?}", expr),
        Err(e) => eprintln!("Parse error: {:?}", e),
    }
}
