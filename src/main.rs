use std::io;

fn main() {
    let x = 5; //immutable variable, but can be shadowed
    println!("value of x is {x}");
    let x = x + 1; //shadow variable, note: if I dont use let it will throw error for modifying immutable variable
    {
        let x = x * 2; //scopes
        println!("value of x in inner scope is {x}");
    }
    println!("value of x after shadowing is {x}");
    let mut y = 20; //mutable variable
    println!("value of y is {y}");
    y = 21;
    println!("value of y after modifying is {y}");
    const Z: u32 = 30; //constant, cannot be used with mut and requires explicit data types, uppercase by convention
    println!("value of Z is {Z}");

    let spaces = "   ";
    let spaces = spaces.len(); //this works, but if we use mut it wont becuse mut does not allow use to mutate datatypes
    println!("value of spaces is {spaces}");

    //Data Types
    let num: u8 = 255; //usigned 8 bit number, range 0 to 255, 256 will result in overflow
                       //unsigned integer  data types are u8, u16, u32, u64, u128, usize is specific to machine architecture
    let num2: i8 = -128; //signed 8 bit integer, range -128 to 127
                         //unsinged integer data types are i8, i16, i32, i128, isize is specific to machine architecture
    let num3: f32 = 3.14; //floating point number
    println!("{}, {}, {}", num, num2, num3);

    //Numeric Operations
    let sum = 5 + 10;
    let diff = 69.54 - 20.10;
    let product = 6 * 9;
    let quotient = 69 / 3;
    let truncated = -5 / 3; //results in -1
    let remainder = 69 % 2;
    println!("sum : {sum}, diff: {diff}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}");

    //Boolean types
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    //Character

    let c = 'z';
    let z: char = 'Z';
    let droplets = '💦';
    println!("c: {c}, z: {z}, droplets: {droplets}");

    //Compund types

    //Tuple
    let tup: (i32, f64, u8) = (500, 1000.0, 69);
    println!("tup: {}", tup.0); // accessing tuple by positional indexing, format string does not support printing it
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}");

    //Array

    let arr = [1, 2, 3, 4];
    let arr: [i8; 4] = [1, 2, 3, 4]; //using datatype
    let arr = [0; 4]; //initialize array of size 4 with all elements 0
    let arr = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    println!("{}", arr[0]);

    //Accessing array and handing errors

    let arr = [1, 2, 3, 4];
    println!("please enter an array index: ");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to readline");
    let index: usize = index.trim().parse().expect("Please enter a number");
    let element = arr[index];
    println!("The value of element at index {index} is {element}");
}
