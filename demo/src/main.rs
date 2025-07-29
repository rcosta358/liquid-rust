use liquid_rust::refine;

fn main() {
    let val1 = refine!("(_ > 0 && _ < 10) || _ == -1", -1); // between 0 and 10 or exactly -1
    let val2 = refine!("_ * 10 < 100", 9); // less than 100 when multiplied by 10
    let val3 = refine!("_ % 2 == 0 ? _ >= 0 : _ < 0", -7); // positive evens or negative odds
    let val4 = refine!("_ == 10 ? true : _ > 0 ? !(_ >= 10) : false", 5); // between 0 and 10

    println!("{}", val1);
    println!("{}", val2);
    println!("{}", val3);
    println!("{}", val4);
}

