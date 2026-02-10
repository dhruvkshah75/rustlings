fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // simply overshadow the the above number varaiable 
    let number: i32 = 3;
    println!("Number plus two is: {}", number + 2);


    // to convert a string into a number we do 
    // let number: u32 = "42".parse().expect("Not a number!") 
    // we use a expect clause as the string might not be converted into a number

    // let guess: u32 = "42".parse().expect("Not a number!");   => we must put in the datatype of the converted string 

}
