fn main() {

    // let a = [2;9];

    // for i in 0..a.len(){
    //     println!("{}",a[i]);
    // }

    // let s1 = String::from("hello");
    // let mut s2 = s1.clone();
    // s2.push_str("a");

    // println!("{} {}", s1, s2);
    // let s = String::from("hello");  // s comes into scope

    // takes_ownership(&s);             // s's value moves into the function...
    //                                 // ... and so is no longer valid here
    //                                 // NOW VALID WITH REF
    
    
    // println!("{}", s);

    // let x = 5;                      // x comes into scope

    // makes_copy(x);                  // x would move into the function,
    //                                 // but i32 is Copy, so it’s okay to still
    //                                 // use x afterward


    // let mut a = "TESTT";
    // for(i,j) in a.chars().enumerate(){
       
    // }
    // println!("{}", a);

    let mut a = "TESTT";
    for i in 0..a.len(){
       print!("{}",i);
       a.replace(i, "a");
    }
    println!("{}", a);

} 

// fn takes_ownership(some_string: &String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.