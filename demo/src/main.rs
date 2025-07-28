use proc_macros::refine;

fn main() {
    let val = refine!("(_ > 0 && _ < 10) || _ == -1", -1);
    println!("{}", val);
}

