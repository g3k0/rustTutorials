fn main() {
    // Adding use and a path in a scope is similar to creating a symbolic link in the filesystem

    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    
    use crate::front_of_house::hosting; // this is an error because use is out of scope
    
    mod customer {
        pub fn eat_at_restaurant() {
            hosting::add_to_waitlist();
        }
    }

    //_--------------------------
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    
    use crate::front_of_house::hosting::add_to_waitlist;
    
    pub fn eat_at_restaurant() {
        add_to_waitlist();
    }

    /**
     * There’s another solution to the problem of bringing two types 
     * of the same name into the same scope with use: after the path, 
     * we can specify as and a new local name, or alias, for the type.
     */
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        // --snip--
    }

    fn function2() -> IoResult<()> {
        // --snip--
    }

    // Re-exporting Names with pub use
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    
    pub use crate::front_of_house::hosting;
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
