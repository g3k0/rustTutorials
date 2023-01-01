fn main() {
    // vectors resides in the heap memory, so the size can change at runtime,
    // vectors are a collectios of items of the same type.
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // this is a macro that permit to initialize a vector with values

    // add values to a vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // readings elements of vector, there are 2 ways
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // with the fitst method rust panicks if we try to extract a non existant 
    // element, with the second method rust returns a None.

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // in this case we want to change the items so we use mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * is a dereference operator
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
