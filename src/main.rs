use std::collections::{HashMap,HashSet};

fn main() {
    // declare a variable
    println!("===============================");
    let name = "Signal Fish";
    let rating = 9.8;  // default f64
    let age = 24;   // default i32
    let man = true;
    let character = "$";
    println!("declare a variable......");
    println!("name = {}", name);
    println!("age = {}", age);
    println!("rating = {}", rating);
    println!("man = {}", man);
    println!("character = {}", character);
    println!("===============================");

    // number saparator
    println!("number saparator");
    let salary:f32 = 10_000.00;
    println!("salary = {}", salary);
    println!("===============================");

    // Constant
    println!("Constant......");
    const PI:f32 = 3.14;
    println!("PI = {}", PI);
    println!("===============================");

    // String Literal
    println!("String Literal......");
    let location:&str = "Guangzhou";
    println!("location = {}", location);
    println!("===============================");

    // String Object
    println!("String Object......");
    let empty_string = String::new();
    let name = String::from("Signal Fish");
    println!("length of empty_string is {}", empty_string.len());
    println!("length of name is {}", name.len());
    println!("===============================");
    
    // match statement
    println!("match statement......");
    let state_code = "MH";
    let state = match state_code {
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "MH" => {println!("Found match for MH"); "Maharashtra"},
        "GA" => "Goa",
        _ => "Unknown"
    };
    println!("State name is {}", state);
    println!("===============================");

    // while 
    println!("while......");
    let mut x = 0;
    while x < 3 {
        x += 1;
        println!("Inside while loop, x value is {}", x);
    }
    println!("Outside while loop, x value is {}", x);
    println!("===============================");

    // loop
    println!("loop......");
    let mut x = 0;
    loop {
        x += 1;
        if x % 2 != 0 {
            continue;
        }
        if x == 10 {
            break;
        }
        println!("x = {}", x);
    }
    println!("===============================");

    // functions
    println!("functions......");
    let num1: i32 = 1;
    mutate_num_to_zero_by_value(num1);
    println!("The value of num1 is {}", num1);
    println!("-------------------------------");
    let mut num2: i32 = 1;
    mutate_num_to_zero_by_reference(&mut num2);
    println!("The value of num2 is {}", num2);
    println!("===============================");

    // tuple
    println!("tuple......");
    let xyz:(f32, f32, bool) = (1.1, 2.2, true);
    let (x, y, z) = xyz;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("===============================");

    // array
    println!("array......");
    let arr: [f32; 3] = [1.1, 2.2, 3.3];
    println!("array is {:?}", arr); 
    println!("array length is {}", arr.len());
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    }
    for value in arr.iter() {
        println!("iterate array with arr.iter() ==> {}", value);
    }
    let arr1 = [1, 2, 3];
    update_array_by_value(arr1);
    println!("Inside main, arr1 = {:?}", arr1);
    let mut arr2 = [1, 2, 3];
    update_array_by_reference(&mut arr2);
    println!("Inside main, arr2 = {:?}", arr2);
    println!("===============================");

    // move
    println!("move......");
    /*
    let v = vec![1, 2, 3];  // vector v owns the object in heap
    let v2 = v;   // moves ownership to v2
    display(v2);   // v2 is moved to display and v2 is invalidated
    println!("In main: v2 = {:?}", v2);  // v2 is no longer usable here
    */
    let v = vec![1, 2, 3];
    let v2 = v;
    let v2_return = return_value(v2);
    println!("In main, v2_return = {:?}", v2_return);
    println!("===============================");

    // borrowing
    println!("borrowing......");
    println!("-------------------------------");
    println!("unmutable borrowing......");
    let v = vec![1, 2, 3];
    print_vector(&v);
    println!("Printing the value from main function, v[0] = {}", v[0]);
    println!("-------------------------------");
    println!("mutable borrowing......");
    let mut name:String = String::from("Rust");
    println!("The value of name before modification is: {}", name);
    mutate_string(&mut name);
    println!("The value of name after modification is: {}", name);
    println!("===============================");

    // slices
    println!("slices......");
    let data = [1, 2, 3, 4, 5];
    use_slice(&data[1..4]);
    println!("-------------------------------");
    println!("mutable slice......");
    let mut data = [333, 666, 999];
    println!("Before mutation, data ={:?}", data);
    mutate_slice(&mut data[..2]);
    println!("After mutation, data ={:?}", data);
    println!("===============================");

    // structure
    println!("structure......");
    let rect = Rectangle::new(10, 20);
    println!("rect = {:#?}", rect);
    println!("width = {};\nheight = {};\narea = {};\nperimeter = {}", rect.width, rect.height, rect.area(), rect.perimeter());
    println!("===============================");

    // enum
    println!("enum......");
    let person1 = Person {
        name: String::from("Signal"),
        gender: Gender::Male
    };
    let person2 = Person {
        name: String::from("Marsha"),
        gender: Gender::Female
    };
    println!("person1 = {:#?}", person1);
    println!("person2 = {:#?}", person2);
    println!("-------------------------------");
    println!("Option enum......");
    match is_even(2) {
        Some(data) => {
            if data == true {
                println!("Even number!");
            }
        },
        None => {
            println!("Odd number");
        }
    }
    println!("===============================");

    // modules
    /*
    pub mod movies {
        pub mod english {
            pub mod comedy {
                pub fn play(name:String) {
                    println!("{}", name);
                }
            }
        }
    }
    use movies::english::comedy
    fn main() {
        comedy::play("Signal");
    }
    */

    // Vector
    let mut v = vec![];  // Vec::new()
    v.push(11);
    v.push(22);
    v.push(33);
    if v.contains(&11) {
        println!("found 11");
    }
    v.remove(1);
    println!("v = {:?}, length of v = {}", v, v.len());
    println!("===============================");

    // Hashmap
    println!("HashMap......");
    let mut state_codes = HashMap::new();
    state_codes.insert("KL", "Kerala");
    state_codes.insert("MH", "Maharashtra");
    println!(" = {:?}", state_codes);
    println!("Size of  is {}", state_codes.len());
    for (key, val) in state_codes.iter() {
        println!("{}: {}", key, val);
    }
    match state_codes.get(&"KL") {
        Some(value) => {
            println!("Value for key KL is {}", value);
        }
        None => {
            println!("nothing found");
        }
    }
    if state_codes.contains_key(&"MH") {
        println!("found key MH");
    }
    println!("===============================");

    // HashSet
    println!("HashSet......");
    let mut nums = HashSet::new();
    nums.insert(1);
    nums.insert(2);
    nums.insert(3);
    nums.insert(1);
    println!("nums = {:?}, length of nums = {}", nums, nums.len());
    for num in nums.iter() {
        println!("{}", num);
    }
    if nums.contains(&1) {
        println!("found 1");
    }
    nums.remove(&2);
    println!("After remove 2, nums = {:?}", nums);
    match nums.get(&2) {
        Some(value) => {
            println!("found {}", value);
        }
        None => {
            println!("not found 2");
        }
    }
    println!("===============================");

    // error handling
    /*
     use std::fs::File
     let f = File::open("a.txt");
     match f {
        Ok(f) => {
            println!("file found {:?}", f);
        }
        Err(e) => {
            println!("file not found\n{:?}", e);
        }
     }
     let f = File::open("a.txt").unwrap();
     let f = File::open("a.txt").expect("File not able to open");
    */

    
}

fn mutate_num_to_zero_by_value(mut param_num: i32) {
    param_num *= 0;
    println!("param_num value is {}", param_num);
}

fn mutate_num_to_zero_by_reference(param_num: &mut i32) {
    // dereference
    *param_num = 0;
    println!("param_num value is {}", param_num);
}

fn update_array_by_value(mut arr:[i32; 3]) {
    for i in 0..arr.len() {
        arr[i] = 0;
    }
    println!("Inside update_array_by_value, arr = {:?}", arr);
}

fn update_array_by_reference(arr: &mut[i32; 3]) {
    for i in 0..arr.len() {
        arr[i] = 0;
    }        
    println!("Inside update_array_by_reference, arr = {:?}", arr);
}

/*
fn display(v:Vec<i32>) {
    println!("Inside display, v = {:?},", v);
}
*/

fn return_value(v:Vec<i32>) -> Vec<i32> {
    v
}

fn print_vector(x:&Vec<i32>) {
    println!("Inside peint_vector function, x = {:?}", x);
}

fn mutate_string(param_name:&mut String) {
    println!("param_name value is {}", param_name);
    param_name.push_str(" Rocks");
}

fn use_slice(slice:&[i32]) {
    println!("slice = {:?}", slice);
}

fn mutate_slice(slice:&mut [i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("slice = {:?}", slice);
    slice[0] = 888;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    // static method
    fn new(w:u32, h:u32) -> Rectangle {
        Rectangle{
            width: w,
            height: h
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

#[derive(Debug)]
enum Gender {
    Male,
    Female
}

#[derive(Debug)]
struct Person {
    name: String,
    gender: Gender
}

fn is_even(num:i32) -> Option<bool> {
    if num % 2 == 0 {
        Some(true)
    } else {
        None
    }
}
