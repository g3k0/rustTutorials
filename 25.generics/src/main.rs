fn main() {
    // Generics are often used to avoid duplicated code, it'sq way to extract 
    // functions that works with differen data types.

    // T is ageneric Type, in this case it can be an i32 or a str
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];
    
        for item in list {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }
    
    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
    
        let result = largest(&number_list);
        println!("The largest number is {}", result);
    
        let char_list = vec!['y', 'm', 'a', 'q'];
    
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    
    // in structs definition
    struct Point<T> {
        x: T,
        y: T,
    }
    
    fn main() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }

    // we can define different types for the struct
    struct Point<T, U> {
        x: T,
        y: U,
    }
    
    fn main() {
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }

    // in enum definition
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // in methods definition
    struct Point<T> {
        x: T,
        y: T,
    }
    
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    
    fn main() {
        let p = Point { x: 5, y: 10 };
    
        println!("p.x = {}", p.x());
    }

    /**
     * Generic type parameters in a struct definition aren’t always 
     * the same as those you use in that same struct’s method 
     * signatures
     */
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }
    
    impl<X1, Y1> Point<X1, Y1> { // the generic parameters X1 and Y1 are declared after impl because they go with the struct definition
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> { // The generic parameters X2 and Y2 are declared after fn mixup, because they’re only relevant to the method.

            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    
    fn main() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
    
        let p3 = p1.mixup(p2);
    
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    // using generic types won't make your program run any slower than it would with concrete types.
}
