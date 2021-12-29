fn main() {
    let number = 3;

    if number < 5 {
      println!("condition was true");
    } else {
      println!("condition was false");
    }

    // `if` espera un boolean y se chequea el tipo en tiempo de ejecución
    // if number {
    //   println!("arm executed")
    // }

    let condition = true;
    // `if` es una expresión así que puede usarse para asignar valor
    let number = if condition {
      5
    } else {
      6
    };

    // Todos los arms de la expresión tienen que devolver el mismo tipo de datos, sino hay error en tiempo de ejecución
    // let number = if condition {
    //   5
    // } else {
    //   "six"
    // };
}
