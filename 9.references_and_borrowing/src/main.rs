fn main() {
    /**
     * A reference is like a pointer in that it’s an address 
     * we can follow to access the data stored at that address; 
     * that data is owned by some other variable. Unlike a pointer, 
     * a reference is guaranteed to point to a valid value of a 
     * particular type for the life of that reference.
     */
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    /*
        The opposite of referencing by using & is dereferencing,
        which is accomplished with the dereference operator, *
     */
    // We call the action of creating a reference borrowing

    change(&s); // this throws an error, references are immutable

    let mut s = String::from("hello");

    change(&mut s); // this is a mutable reference

    /*
        Mutable references have one big restriction: 
        if you have a mutable reference to a value, you can 
        have no other references to that value. This code that 
        attempts to create two mutable references to s will fail
    */

    let r1 = &mut s;
    let r2 = &mut s; // error, we can't have 2 mutable references for the same value in the same scope

    // this is ok, r1 is in its own scope
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // we can't have mutable references when for that value we defined immutable references
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    // Users of an immutable reference don’t expect the value to suddenly change out from under them!

    // this is ok
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &String) {
    some_string.push_str(", world");
}
