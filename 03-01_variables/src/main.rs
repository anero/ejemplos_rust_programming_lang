const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of X is: {}", x);
    x = 6;
    println!("The value of X is: {}", x);

    let x = x + 1;
    let x = x * 2;

    println!("The value of X is: {}", x);

    let n: u32 = "42".parse().expect("Not a number!"); // necesito definir el tipo porque no se puede inferir en timpo de compilación

    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);
    println!("The value of y is: {}", tup.1); // pattern matching

    // Array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
