fn main() {
    another_function(5);

    // Statements
    let x = 5; // `let` es un statement, no devuleve valor. ej: `let x = (let y = 1);` tira error de compilacion

    // Expressions
    let x = 5; // `5` "devuelve" el valor 5
    let y = {     // Un scope es una expresión si la última línea no tiene `;`
      let x = 3;
      x + 1       // sin `;` para que devuelva el valor y el scope sea una expresión y no un statement
    };
    println!("The value of y is: {}", y);

    let x = plus_one(5);
    println("The value of x is: {}", x);
}

// La función puede declararse antes o después de las invocaciones
fn another_function(x: i32) { // Siempre hay que anotar el tipo de los parámetros
  println!("The value of x is: {}", x);
}

fn plus_one(x: u32) -> u32 {  // Hay que anotar el tipo del dato devuelto
  x + 1 // expresión devuelta implícitamente
}