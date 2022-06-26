/*
    extern crate rand;
    use std::io;
    use rand::Rng;
    use std::cmp::Ordering;
*/
fn func(x:u32) {
    println!("{}", x);
}
fn main() {
    println!("Hello world!");
    func(1);
/*
    println!("Input whatever you want:");
    let rand = rand::thread_rng().gen_range(1,101);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to input!");
        let num:u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };
        match num.cmp(&rand) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("Same!");break;}
        }
    }
*/
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Element is {}!",element);
    }
}
