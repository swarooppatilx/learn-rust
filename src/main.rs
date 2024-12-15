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
}
