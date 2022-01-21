#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker,
    Other(String),
}

#[derive(Debug)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print(emp: Employee) {
    println!("{:?}", emp)
}

fn main() {
    let me = Employee {
        position: Position::Other("Vagabundo".to_owned()),
        work_hours: 80,
    };

    print(me);
}
