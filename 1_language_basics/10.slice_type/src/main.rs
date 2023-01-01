fn main() {
    /*
    Slices let you reference a contiguous sequence of elements
    in a collection rather than the whole collection. A slice is a 
    kind of reference, so it does not have ownership.
     */

     let mut s = String::from("hello world");

     let word = first_word(&s); // word will get the value 5
 
     s.clear(); // this empties the String, making it equal to ""
 
     // word still has the value 5 here, but there's no more string that
     // we could meaningfully use the value 5 with. word is now totally invalid!

    // slice
    let s = String::from("hello world");

    let hello = &s[0..5]; //  We create slices using a range within brackets by specifying [starting_index..ending_index]
    let world = &s[6..11];

    // string slices as parameters
    /**
     * If we have a string slice, we can pass that directly. 
     * If we have a String, we can pass a slice of the String or 
     * a reference to the String. This flexibility takes advantage 
     * of deref coercions
     */
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// rewrite the function with slices
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// slices as parameters
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}