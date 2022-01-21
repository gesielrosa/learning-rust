// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Client {
    age: i32
}

fn can_purchase(client: &Client) -> Result<(), String> {
    if client.age >= 21 {
        Ok(())
    } else {
        Err("Client under age".to_owned())
    }
}

fn main() {
    let client = Client {
        age: 23
    };

    let order = can_purchase(&client);
    match order {
        Ok(_) => println!("Client can purchase"),
        Err(e) => println!("Client can't purchase reason: {:?}", e)
    }
}
