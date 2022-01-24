// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use ::std::collections::HashMap;
use ::std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    list: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            list: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill) -> Result<(), String> {
        if self.list.contains_key(&bill.name) {
            Err("[Fail] Account already exists".to_owned())
        } else {
            self.list.insert(bill.name.clone(), bill);
            Ok(())
        }
    }

    fn get_all(&mut self) -> Vec<&Bill> {
        let mut bills: Vec<&Bill> = Vec::new();
        for bill in self.list.values() {
            bills.push(bill)
        }
        bills
    }

    fn remove(&mut self, name: &str) -> Result<(), String> {
        if self.list.contains_key(name) {
            self.list.remove(name);
        }
        Err("[Fail] Account not found".to_owned())
    }

    fn update(&mut self, name: &str, amount: &f64) -> Result<(), String> {
        match self.list.get_mut(name) {
            Some(inner_bill) => {
                inner_bill.amount = amount.to_owned();
                Ok(())
            }
            None => Err("[Fail] Account not found".to_owned())
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn show_menu() {
    println!("=== Main menu ===");
    println!("[1] Add");
    println!("[2] View all");
    println!("[3] Remove");
    println!("[4] Edit");
    println!("Enter option number:")
}

fn add_menu(bills: &mut Bills) {
    let name = match get_input() {
        Ok(input) => input,
        Err(_) => ""
    };
    println!("[1] Add");
}

fn view_all_menu(bills: &Bills) {}

fn remove_menu(bills: &mut Bills) {}

fn edit_menu(bills: &mut Bills) {}

fn main() {
    let mut bills = Bills::new();
    loop {
        show_menu();
        match get_input() {
            Ok(input) => {
                match input.as_str() {
                    "1" => add_menu(&mut bills),
                    "2" => view_all_menu(&bills),
                    "3" => remove_menu(&mut bills),
                    "4" => edit_menu(&mut bills),
                    _ => break
                }
            }
            Err(_) => break
        }
    }
}
