fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn build_user2(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    /**
     * In this example, we can no longer use user1 after 
     * creating user2 because the String in the username 
     * field of user1 was moved into user2
     */
    let user4 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // tuple structs
    /**
     *  Tuple structs are useful when you want to give the 
     * whole tuple a name and make the tuple a different type 
     * from other tuples, and when naming each field as in a 
     * regular struct would be verbose or redundant.
     */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
    /**
     * Note that the black and origin values are different 
     * types, because they’re instances of different tuple 
     * structs.
     */

     // ou can also define structs that don’t have any fields! These are called unit-like structs 
    struct AlwaysEqual;

    fn main() {
        let subject = AlwaysEqual;
    }
}
