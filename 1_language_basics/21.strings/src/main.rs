fn main() {
    // text is managed by String type and str string slice, 
    // they are a collection similar to vectors
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // a string can grow
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // strings concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // we can only add a &str to a String; we can’t add two String values together
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // we use the format macro to concatenate multiple strings
    let s = format!("{s1}-{s2}-{s3}");

    // indexing:  Rust strings don’t support indexing
    // because an index into the string’s bytes will not always correlate to a valid 
    // Unicode scalar value

    /**
     * Another point about UTF-8 is that there are actually three 
     * relevant ways to look at strings from Rust’s perspective: 
     * as bytes, scalar values, and grapheme clusters 
     * (the closest thing to what we would call letters).
     */

     // Iterating over strings
     for c in "Зд".chars() {
        println!("{c}");
    }

    // this returns the bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

}
