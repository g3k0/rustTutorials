fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // we could write this in a shorter way using if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    /**
     * you can think of if let as syntax sugar for a match 
     * that runs code when the value matches one pattern and 
     * then ignores all other values.
     */

     // We can include an else with an if let
     let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // the code above is equalen to tehe following
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

}
