fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // but the type can't change
    println!("The value of x is: {x}");

    /**
     * First, you aren’t allowed to use mut with constants. 
     * Constants aren’t just immutable by default—they’re 
     * always immutable. 
     * You declare constants using the const keyword 
     * instead of the let keyword, and the type 
     * of the value must be annotated. 
     * Just know that you must always annotate the type.
     */

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
