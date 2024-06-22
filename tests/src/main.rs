fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");


    let mut y: u8 = 254;
    let z: Option<u8> = y.checked_add(1);

    y += 1;
    y = y.wrapping_add(1);
    println!("{:?}, {:?}", y, z);


    let arr: [u8; 5] = [0; 5];
    println!("{:?}", arr);

    let number = 3;

    if number < 10 {
        println!("number is below 10");
    } else if number < 5 {
        println!("number is below 5");
    } else {
        println!("number is above 10");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of the number is : {number}");
}
