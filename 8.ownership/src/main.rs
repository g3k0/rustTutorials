fn main() {
    /**
     * As a first example of ownership, we’ll look at the scope 
     * of some variables. A scope is the range within a program 
     * for which an item is valid. Take the following variable
     */

     // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward

    let mut s = String::from("hello");
    // This kind of string can be mutated

    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    /**
     * To ensure memory safety, after the line let s2 = s1;, 
     * Rust considers s1 as no longer valid. Therefore, 
     * Rust doesn’t need to free anything when s1 goes out of 
     * scope
     */
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // it throws an error

    /**
     * If we do want to deeply copy the heap data of the String,
     * not just the stack data, we can use a common method
     * called clone
     */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    /**
     * This does not throws an error because the variable
     * has a fixed size, and in this case is entirely stored
     * in the stack memory, not the heap.
     * For such king of variables, there no difference between 
     * a shallow copy and a deep copy.
     */
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    /**
     * Rust has a special annotation called the Copy trait 
     * that we can place on types that are stored on the stack,
     * as integers are. If a type implements the Copy trait, 
     * variables that use it do not move,
     * but rather are trivially copied, making them still 
     * valid after assignment to another variable.
     */

     // Ownership and functions
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                     // ... and so is no longer valid here
    
    // If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error

    let x = 5;                      // x comes into scope
 
    makes_copy(x);                  // x would move into the function,
                                     // but i32 is Copy, so it's okay to still
                                     // use x afterward

    // Return values and scope
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3




    /**
     * The ownership of a variable follows the same 
     * pattern every time: assigning a value to another 
     * variable moves it. When a variable that includes data 
     * on the heap goes out of scope, the value will be 
     * cleaned up by drop unless ownership of the data has 
     * been moved to another variable.
     */

    // Rust does let us return multiple values using a tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);


} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.
// this scope is now over, and s is no longer valid

/**
 * There is a natural point at which we can return the memory 
 * our String needs to the allocator: when s goes out of scope. 
 * When a variable goes out of scope, Rust calls a special 
 * function for us. This function is called drop, and it’s 
 * where the author of String can put the code to return the memory. 
 * Rust calls drop automatically at the closing curly bracket.
 */

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}