// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

fn print_name(name: &str) {
    println!("Name = {:?}", name)
}

fn print_color(color: &str) {
    println!("Color = {:?}", color)
}

fn main() {
    let people = vec![
        Person {
            age: 5,
            name: "Carlos".to_owned(),
            fav_color: "Red".to_owned(),
        },
        Person {
            age: 15,
            name: "Artur".to_owned(),
            fav_color: "Blue".to_owned(),
        },
        Person {
            age: 3,
            name: "Ana".to_owned(),
            fav_color: "Yellow".to_owned(),
        },
    ];

    for person in &people {
        if person.age <= 10 {
            print_name(&person.name);
            println!("Age = {:?}", person.age);
            print_color(&person.fav_color);
            println!();
        }
    }
}
