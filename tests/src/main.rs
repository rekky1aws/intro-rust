fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");


    // Unisigned int looping
    let mut y: u8 = 254;
    let z: Option<u8> = y.checked_add(1);

    y += 1;
    y = y.wrapping_add(1);
    println!("{:?}, {:?}", y, z);


    // Arrays
    let arr: [u8; 5] = [0; 5];
    println!("{:?}", arr);


    // Conditions
    let number = 3;

    if number < 10 {
        println!("number is below 10");
    } else if number < 5 {
        println!("number is below 5");
    } else {
        println!("number is above 10");
    }


    // If statement in variable definition
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of the number is : {number}");


    // Infintie loop
    /*
    loop {
        println!("again !");
    }
    */


    // Looping
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    // Labelling loops
    println!("\n\n\n\n\n");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}", );
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
