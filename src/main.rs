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
//let k: i16 = 100_000_000;

println!("{}",adds_three(5));

//yay f#cking ternary
// let is_below_eigthteen = if age > 18 {true} else {false};

// we also have case kinda with "match"

//Trying strucs, impls and traits
struct Player {
    first_name: String,
    last_name: String,
}
impl Player {
    fn get_name(&self) -> String {
       return format!("{} {}", self.first_name, self.last_name);           
    }
}
let player_1 = Player {
    first_name: "Danilo".to_string(),
    last_name: "Diez".to_string(),
};

println!("Player 01: {}", player_1.get_name());
let vector: Vec<i32> = vec![12, 32, 33, 43];
map_vector(vector);
}


// We set the type of a variable with ":"
fn adds_three(a: i8) -> i8 {
    return a + 3;
}

fn map_vector(v: Vec<i32>){
    for e in v {
        println!("{}", e)
    }
}


