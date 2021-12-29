fn main() {
    // Ciclo infinito sin condición de corte
    // loop {
    //   println!("again!"); // se puede cortar con `break`
    //
    // }

    let mut number = 3;

    while number > 0 {
      println!("{}", number);
      number = number - 1;
    }
    println!("LIFTOFF!");

    // `for` para iterar sobre array
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
      println!("the value is: {}", element);
    }

    // Ciclo usando `for` con un Range
    for number in (1..4).rev() {
      println!("{}!", number);
    }
}
