// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Colors {
    Red,
    White,
    Blue,
}

impl Colors {
    fn print(&self) {
        match self {
            Colors::Red => println!("Color: Red"),
            Colors::White => println!("Color: White"),
            Colors::Blue => println!("Color: Blue")
        };
    }
}

struct Dimension {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimension {
    fn print(&self) {
        println!("Width: {:?}, Height: {:?}, Depth: {:?}", self.width, self.height, self.depth);
    }
}

struct ShippingBox {
    dimensions: Dimension,
    weight: f64,
    color: Colors,
}

impl ShippingBox {
    fn new() -> Self {
        Self {
            dimensions: Dimension {
                width: 50.0,
                height: 30.2,
                depth: 80.60
            },
            weight: 5.3,
            color: Colors::Red,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight)
    }
}

fn main() {
    let shipping_box = ShippingBox::new();
    shipping_box.print();
}
