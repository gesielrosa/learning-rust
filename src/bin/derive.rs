#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print(emp: Employee) {
    println!("{:?}", emp)
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 80,
    };

    print(me);
    print(me);
}
