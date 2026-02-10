// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i64 {
    // this is the return statement so remove the semicolon 
    (num * num) as i64    // => this is the type conversion that is important 
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
