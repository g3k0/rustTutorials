use std::fs::File;
use std::io::{self, Read};


fn main() {
    /**
     * that the Result enum is defined as having two variants, 
     * Ok and Err, as follows:
     */
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let greeting_file_result = File::open("hello.txt");

    /**
     * In the case where File::open succeeds, the value in the 
     * variable greeting_file_result will be an instance of Ok 
     * that contains a file handle
     */
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // matching on different errors
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Shortcuts for Panic on Error: unwrap and expect

    /**
     * unwrap: If the Result value is the Ok variant, unwrap will 
     * return the value inside the Ok. If the Result is the Err 
     * variant, unwrap will call the panic! macro for us
     */
    let greeting_file = File::open("hello.txt").unwrap();

    // expect
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    /**
     * In production-quality code, most Rustaceans choose expect 
     * rather than unwrap and give more context about why the 
     * operation is expected to always succeed. That way, 
     * if your assumptions are ever proven wrong, you have more 
     * information to use in debugging.
     */

     // propagating errors
     fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
    
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut username = String::new();
    
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // A Shortcut for Propagating Errors: the ? Operator
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?; //expecially used if the operation can fail for many reasons
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    /**
     * The ? operator can only be used in functions whose return 
     * type is compatible with the value the ? is used on
     * (Result, Option, or another type that implements FromResidual)
     */

     /**
      * panic! is often appropriate if you’re calling external code 
      that is out of your control and it returns an invalid state 
      that you have no way of fixing.
      However, when failure is expected, it’s more appropriate 
      
      to return a Result than to make a panic! call 
      */

}
