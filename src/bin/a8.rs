// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


struct Drink {
    flavors: Flavors,
    once: f64,
}

enum Flavors {
    Pineapple,
    Apple,
    Strawberry,
}

fn print_drink(my_drink: Drink) {
    println!("Drink properties");
    println!("{:?} fl oz", my_drink.once);

    match my_drink.flavors {
        Flavors::Pineapple => println!("Pineapple"),
        Flavors::Apple => println!("Apple"),
        Flavors::Strawberry => println!("Strawberry")
    }
}

fn main() {
    let my_drink = Drink {
        flavors: Flavors::Apple,
        once: 5.1,
    };

    print_drink(my_drink)
}
