// fn main() {
//     println!("Hello, world!");

//     another_function(5);

//     print_labeled_measurement(15, 'k');
// }

// fn another_function(x:i32) {
//     println!("Another function. With value {x}");
// }

// fn print_labeled_measurement(value: u32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");

    let y = plus_one(10);

    println!("y = {y}");

    println!("plus_one(20) = {}", plus_one(20));
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // return  x + 1;
}