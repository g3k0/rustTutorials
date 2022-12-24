fn main() {
    // The Option type encodes the very common scenario in 
    // which a value could be something or it could be nothing

    enum Option<T> {
        None,
        Some(T),
    } // this is the implementation of Option, because in rust there is no the null value

    // you can use Some and None directly without the Option:: prefix. The Option<T>

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

     // you have to convert an Option<T> to a T before you can perform T operations with it
}


