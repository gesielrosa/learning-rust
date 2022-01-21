// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeRole {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    Kitchen,
    Technician,
}

enum EmploymentStatus {
    Employed,
    Unemployed,
}

struct Employee {
    role: EmployeeRole,
    status: EmploymentStatus,
}

fn try_enter(employee: &Employee) -> Result<(), String> {
    match employee.status {
        EmploymentStatus::Unemployed => return Err("unemployed".to_owned()),
        _ => ()
    }

    match employee.role {
        EmployeeRole::Maintenance => Ok(()),
        EmployeeRole::Marketing => Ok(()),
        EmployeeRole::Manager => Ok(()),
        _ => Err("Role cant access".to_owned())
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = try_enter(employee)?;
    println!("Access ok");
    Ok(())
}

fn main() {
    let employee = Employee {
        role: EmployeeRole::Manager,
        status: EmploymentStatus::Employed
    };

    match print_access(&employee) {
        Err(e) => println!("access denied: {:?}", e),
        _ => ()
    }
}
