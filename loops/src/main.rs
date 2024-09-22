fn main() {
    // # Code 1
    // loop {
    //     println!("again!");
    // }

    // # Code 2
    // let mut counter = 0;

    // let number = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("number = {}", number);

    // # Code 3
    let mut count = 0;
    let ret = 'count_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_up 15;
            }
            remaining -= 1;
        }
        count += 1;
    };

    println!("ret = {ret}");
}
