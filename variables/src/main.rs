fn main() {
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("x in inner block {x}");
    // }
    // println!("x in outer block is {x}");
    let mut x = (200, 56.7, 1u8);
    let (_a, b, _) = x;
    x.2 = 7;
    let c = x.2;
    println!("c = {c}");
    println!("2nd value is {}", x.1);
    println!("value of b is {b}");
}
