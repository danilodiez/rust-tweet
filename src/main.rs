fn main() {
    println!("Hello, world!");
let a;
a = 5;

let b: i8; //declaration with type
b = 5;

let b: bool = false; 

let (x, y) = (4, 5);

let mut z: i8 = 2;

z = 3;
// we can use _ to divide long numbers like
let k: i16 = 100_000_000;

println!("{}",adds_three(5));

//yay f#cking ternary
// let is_below_eigthteen = if age > 18 {true} else {false};

// we also have case kinda with "match"

}


// We set the type of a variable with ":"
fn adds_three(a: i8) -> i8 {
    return a + 3;
}

