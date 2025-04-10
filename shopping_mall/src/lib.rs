pub mod mall;

pub use floor::store;
pub use mall::floor;
pub use mall::guard;
pub use mall::*;
pub use store::employee;

pub fn biggest_store(mall: mall::Mall) -> store::Store {
    let mut res: store::Store = store::Store::new("", 0, vec![]);

    for floor_a in mall.floors.iter() {
        for shop in floor_a.stores.iter() {
            if shop.square_meters > res.square_meters {
                res = shop.clone();
            }
        }
    }

    res
}

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<employee::Employee> {
    let mut res = vec![employee::Employee::new("", 0, 0, 0, 0.0)];
    for elem in mall.floors.iter() {
        for shop in elem.stores.iter() {
            for emp in shop.employees.clone().into_iter() {
                if emp.salary > res[0].salary {
                    res[0] = emp.clone();
                } else if emp.salary == res[0].salary {
                    res.push(emp.clone());
                }
            }
        }
    }
    res
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut res = 0;

    for floor_a in mall.floors.iter() {
        for shop in floor_a.stores.iter() {
            res += shop.employees.len();
        }
    }

    res + mall.guards.len()
}

pub fn check_for_securities(mall: &mut mall::Mall, available_sec: Vec<guard::Guard>) {
    let mut size = 0;
    for floor in mall.floors.iter() {
        size += floor.size_limit;
    }

    let mut i = 0;
    while (mall.guards.len() as f64) < size as f64 / 200.0 {
        mall.hire_guard(available_sec[i].clone());
        i += 1;
    }
}

pub fn cut_or_raise(mall: &mut mall::Mall) {
    for (i, elem) in mall.clone().floors.iter().enumerate() {
        for (j, shop) in elem.stores.iter().enumerate() {
            for (z, emp) in shop.employees.iter().enumerate() {
                if emp.working_hours.1 - emp.working_hours.0 >= 10 {
                    mall.floors[i].stores[j].employees[z].raise(emp.salary * 0.1);
                } else {
                    mall.floors[i].stores[j].employees[z].cut(emp.salary * 0.1);
                }
            }
        }
    }
}


// pub mod mall;
// pub use mall::*;
// pub use mall::{Mall, guard::Guard, floor::store::Store, floor::store::employee::Employee};

// pub fn biggest_store(mall: Mall) -> Store {
//     mall.floors.iter()
//         .flat_map(|floor| floor.stores.iter())
//         .max_by_key(|store| store.square_meters)
//         .unwrap()
//         .clone()
// }

// pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
//     let all_emps: Vec<Employee> = mall.floors.iter()
//         .flat_map(|floor| floor.stores.iter())
//         .flat_map(|store| store.employees.iter().cloned())
//         .collect();

//     if let Some(max_salary) = all_emps.iter().map(|e| e.salary).max_by(|a, b| a.partial_cmp(b).unwrap()) {
//         all_emps.into_iter()
//             .filter(|e| e.salary == max_salary)
//             .collect()
//     } else {
//         vec![]
//     }
// }

// pub fn nbr_of_employees(mall: Mall) -> usize {
//     mall.floors.iter()
//         .flat_map(|floor| floor.stores.iter())
//         .map(|store| store.employees.len())
//         .sum::<usize>()
//         + mall.guards.len()
// }

// pub fn check_for_securities(mall: &mut Mall, mut new_guards: Vec<Guard>) {
//     let total_area: u64 = mall.floors.iter()
//         .flat_map(|floor| floor.stores.iter())
//         .map(|store| store.square_meters)
//         .sum();

//     let required_guards = (total_area as f64 / 200.0).ceil() as usize;
//     let current_guards = mall.guards.len();

//     if required_guards > current_guards {
//         let needed = required_guards - current_guards;
//         for guard in new_guards.drain(..needed.min(new_guards.len())) {
//             mall.hire_guard(guard);
//         }
//     }
// }

// pub fn cut_or_raise(mall: &mut Mall) {
//     for floor in mall.floors.iter_mut() {
//         for store in floor.stores.iter_mut() {
//             for emp in store.employees.iter_mut() {
//                 let hours = emp.working_hours.1 - emp.working_hours.0;
//                 if hours > 10 {
//                     emp.raise(emp.salary * 0.10);
//                 } else {
//                     emp.cut(emp.salary * 0.10);
//                 }
//             }
//         }
//     }
// }

// pub mod mall;
// pub use mall::*;
// // Import the Mall struct and its associated modules and structs
// pub use mall::{Mall, guard::Guard, floor::store::Store, floor::store::employee::Employee};

// pub fn biggest_store(mall: &Mall) -> &Store {
//     mall.floors.iter()
//         .flat_map(|floor| &floor.stores)
//         .max_by_key(|store| store.square_meters)
//         .expect("The mall should have at least one store.")
// }

// pub fn highest_paid_employee(mall: &Mall) -> Vec<&Employee> {
//     let mut highest_paid = Vec::new();
//     let mut max_salary = 0.0;

//     for floor in &mall.floors {
//         for store in &floor.stores {
//             for employee in &store.employees {
//                 if employee.salary > max_salary {
//                     highest_paid.clear();
//                     highest_paid.push(employee);
//                     max_salary = employee.salary;
//                 } else if employee.salary == max_salary {
//                     highest_paid.push(employee);
//                 }
//             }
//         }
//     }

//     highest_paid
// }

// pub fn nbr_of_employees(mall: &Mall) -> usize {
//     let mut count = mall.guards.len();

//     for floor in &mall.floors {
//         for store in &floor.stores {
//             count += store.employees.len();
//         }
//     }

//     count
// }

// pub fn check_for_securities(mall: &mut Mall, new_guards: Vec<Guard>) {
//     let total_square_meters: u64 = mall.floors.iter()
//         .map(|floor| floor.size_limit)
//         .sum();

//     let required_guards = (total_square_meters + 199) / 200;

//     if mall.guards.len() < required_guards as usize {
//         for guard in new_guards {
//             mall.hire_guard(guard);
//             if mall.guards.len() >= required_guards as usize {
//                 break;
//             }
//         }
//     }
// }


// pub fn cut_or_raise(mall: &mut Mall) {
//     for floor in &mut mall.floors {
//         for store in &mut floor.stores {
//             for employee in &mut store.employees {
//                 let working_hours = (employee.working_hours.1 - employee.working_hours.0) as f64;
//                 if working_hours > 10.0 {
//                     employee.raise(employee.salary * 0.1);
//                 } else {
//                     employee.cut(employee.salary * 0.1);
//                 }
//             }
//         }
//     }
// }
