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

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

}
