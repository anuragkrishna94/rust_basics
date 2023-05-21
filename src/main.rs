use std::io;

fn main() {
    print_distance(10, 'm');
    let double_value = return_double(10);
    print_distance(double_value, 'm');
    let a: [u32;5] = [1, 2, 3, 4, 5];
    println!("Enter arr index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Input isn't a number!");
    let element = a[index];
    println!("Value of at {index} is: {element}");

    loop_through_array();
    string_copy_one();
    stack_copy_one();

    // Ownership and Borrowing demonstration
    let s: String = String::from("Namaste");
    takes_ownership(s);
    let mut s1 = gives_ownership_to_caller();
    println!("{s1}");
    s1 = takes_and_gives_back_ownership(s1);
    println!("s1: {s1}");
    let (s2, len) = calc_len_without_ref(s1);
    println!("Length of '{s2}', is {len}");
    let mut s3 = String::from("Namste, Everyone");
    let len1 = calc_len_with_ref(&s3);
    println!("Length of '{s3}', is {len1}");
    let len_of_mutated_s3 = mutate_string_calc_len_ref(&mut s3);
    println!("Length of '{s3}' is {len_of_mutated_s3}");

    // Slices
    let s4 = String::from("How are you?");
    let word_one: &str = get_first_word_slices(&s4);
    println!("First word in '{s4}' is '{word_one}'");

    // String literals are also slices
    let s5: &str = "Namaskaram Everyone";
    let word_one_s5 = get_first_word_slices(s5);
    println!("First word in '{s5}' is '{word_one_s5}'");
}

// The following function is a statement. That is it doesn't evaluate to a value.
fn print_distance(value: i32, units: char) {
    println!("Distance is : {value}{units}");
}

// The following function ends with an expression hence evaluates to a value and that is returned.
fn return_double(x: i32) -> i32 {
    x * 2
}

// for is faster than while index < 5.
// In the case of while, every iteration requires the condition to be re-evaluated.
fn loop_through_array() {
    let a: [i32;5] = [23,45,12,76,12];
    for element in a {
        println!("value is : {element}");
    }
}

fn string_copy_one() {
    let mut s1 = String::from("Namaste");
    // In the line below, s1 is moved into s2. It's NOT a shallow copy. It is a total move altogether.
    let s2 = s1;
    // After the above line is executed, Rust considers s1 as no longer valid.
    s1 = String::from("Namaskaram");
    println!("Before | s1 : {s1}, s2: {s2}");
}

fn stack_copy_one() {
    let x = 10;
    let y = x;
    println!("x: {x}, y: {y}");
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn gives_ownership_to_caller() -> String {
    let s = String::from("Everyone");
    s
}

fn takes_and_gives_back_ownership(s: String) -> String {
    s.to_uppercase()    
}

// In this case, parameter 's' takes ownership of the argument string that is passed.
fn calc_len_without_ref(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// In this case, parameter 's', which is a reference to the argument passed, doesn't take any ownership of the argument that is passed.
fn calc_len_with_ref(s: &String) -> usize {
    s.len()
}

fn mutate_string_calc_len_ref(s: &mut String) -> usize {
    s.push_str(", Wassup");
    s.len()
}

// Uncomment the following code to see how Rust compiler prevents dangling references
// fn try_to_create_dangling_reference() -> &String {
//     let s = String::from("Namaste");
//     &s
// }

fn get_first_word_slices(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &element_ref) in bytes.iter().enumerate() {
        if element_ref == b' ' {
            return &s[..index];
        }
    }
    &s[..]
}