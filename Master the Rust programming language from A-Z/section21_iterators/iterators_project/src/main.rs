#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

fn main() {
    let mut points = [3, 8, 1, 11, 5];
    println!("{}", points.is_sorted());

    points.sort();

    println!("{points:?}");
    println!("{}", points.is_sorted());

    points.reverse();

    println!("{points:?}");
    println!("{}", points.is_sorted());

    let mut exercises = ["squat", "bench", "deadlift"];
    exercises.sort();
    println!("{exercises:?}");





    let mobal = GasStation {
        snack_count: 100,
        manager: String::from("Tyson Masha"),
        employee_count: 3,
    };

    let exxon = GasStation {
        snack_count: 130,
        manager: String::from("Bafana Maja"),
        employee_count: 4,
    };

    let shell = GasStation {
        snack_count: 50,
        manager: String::from("Khomotxo mnisi"),
        employee_count: 2,
    };








}
