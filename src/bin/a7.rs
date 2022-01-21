// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Black,
    White,
    Red,
    Blue,
}

fn print_color(color: Colors) {
    match color {
        Colors::Black => println!("Black"),
        Colors::White => println!("White"),
        Colors::Red => println!("Red"),
        Colors::Blue => println!("Blue")
    }
}

fn main() {
    let color = Colors::Black;
    print_color(color)
}
