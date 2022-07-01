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

println!("{}",adds_three(5));
}
// We set the type of a variable with ":"
fn adds_three(a: i8) -> i8 {
    return a + 3;
}
