fn calculate_length(s: String) -> (String, usize) {
 let length = s.len();
 (s, length) //multiple returns
}

fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String){
    s.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("dangle");
    &s //throws error as s is deallocated after this function
}

fn main() {
    let _a = 10; //stays in scope of main()
    {
        let b = "xyz";
    } //b is invalid after this this, and freed from memory
    println!("{_a}"); //valid
                     //println!("{b}"); //throws error
                     //let s = "Hello"; //immutable string literal
                     // let s = s + ", world"; //this is not allowed, as string literals are hardcoded and value is known at compile time
                     //let s = String::from("Hello"); //immutable
                     //s.push_str(", world"); //throws error
                     //Right approach is
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{}", s);

    let x = 10; //by adding underscore we tell the compiler this varible is intentionally unused
    let y = x; //the value of x is copied into y

    println!("x: {x}, y: {y}"); //data types with known size are stored on stack, hence they are quick to copy so this is valid

    //In case of String, they are stored in heap and are shallow copied by assignment syntax

    let s1 = String::from("some text");
    let s2 = s1; //s1 

    //println!("{s1}"); //throws error as s1 is deallowcated and treated as invalid
    println!("{s2}"); //rust solves the problem of double free error by only keeping s2 as valid reference

    //This behaviour can be avoided by deep copying

    let s3 = String::from("lorem ipsum");
    let s4 = s3.clone(); //deep copy

    println!("s3: {s3}, s4: {s4}");

    //return ownership of parameters
    let s1 = String::from("abc");
    let (s1, len) = calculate_length(s1);
    println!("The length of '{s1}' is {len}");
    println!("The length of '{s1}' using reference is {}", calculate_length_by_ref(&s1));

    //mutable references
    //similar to variables, references are immutable by default

    let mut s = String::from("Hello in mutable reference");
    change(&mut s); //mutable reference, throws error if we try to use immutable reference
    println!("{s}");

    //multiple mutable references
    let mut s = String::from("Hello");
    let r1 = &mut s;
    //let r2 = &mut s; //throws error as we cannot have multiple mutable references
    //println!("{r1}, {r2}");

    //use scope to avoid multiple mutable references
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
        println!("{r1} in scoped reference");
    }
    let r2 = &mut s;
    println!("{r2} in second reference");

    //combination of mutable and immutable references
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; //throws error as we cannot have mutable reference when we have immutable references
    println!("{r1}, {r2}"); //no problem in having multiple immutable references

    let r3 = &mut s;
    println!("{r3}"); //no problem in having mutable reference after dropping immutable references

    //dangling references
    
    //let reference = dangle();
    //println!("{reference}");
}
