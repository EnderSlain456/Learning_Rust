pub fn run () {
    let mut x = 5;
    let y = 10;
    let z = 15;


    println!("Initial values: {} {} {}", x,  y, z);


    x = 20;
    println!("Updated x: {}", x);

    const MAX_POINTS: i32 = 100;
    println!("Constant value: {}", MAX_POINTS);

    let is_active: bool = true;
    let letter: char = 'R';
    let pi: f64 = 3.14159;


    println!("Bool {}, Char: {}, Float: {}", is_active, letter, pi);
}