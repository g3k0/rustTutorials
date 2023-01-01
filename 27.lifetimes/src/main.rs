fn main() {
    /**
     * Lifetimes are another kind of generic that we’ve already 
     * been using. Rather than ensuring that a type has the behavior 
     * we want, lifetimes ensure that references are valid as long as 
     * we need them to be.
     */

    // Preventing Dangling References with Lifetimes
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r); // r has gone out of scope

    // rewrite
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r);

    // generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Lifetime Annotation Syntax

    /**
     * ust as functions can accept any type when the signature specifies 
     * a generic type parameter, functions can accept references 
     * with any lifetime by specifying a generic lifetime parameter
     */
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime

    /**
     * Lifetime Annotations in Struct Definitions
     */
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    /**
     * The main function here creates an instance of the ImportantExcerpt 
     * struct that holds a reference to the first sentence of the String 
     * owned by the variable novel. The data in novel exists before the 
     * ImportantExcerpt instance is created. In addition, novel 
     * doesn’t go out of scope until after the ImportantExcerpt goes 
     * out of scope, so the reference in the ImportantExcerpt instance 
     * is valid.
     */
    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    // Lifetime Annotations in Method Definitions
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // The Static Lifetime
    /**
     * One special lifetime we need to discuss is 'static, which 
     * denotes that the affected reference can live for the entire 
     * duration of the program
     */
    let s: &'static str = "I have a static lifetime.";
    // he text of this string is stored directly in the program’s binary, which is always available

}

// When annotating lifetimes in functions, the annotations go in the function signature, not in the function body
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
