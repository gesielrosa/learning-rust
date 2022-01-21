// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut total_items: i32 = 0;
    let mut stock = HashMap::new();
    stock.insert("chair", 5);
    stock.insert("bed", 3);
    stock.insert("table", 2);
    stock.insert("couches", 0);

    for (name, stock) in stock.iter() {
        print!("{:?} = ", name);
        match stock {
            0 => print!("out of stock"),
            _ => {
                total_items += stock;
                print!("{:?}", stock)
            }
        }
        println!()
    }
    println!("Total items = {:?}", total_items)
}
