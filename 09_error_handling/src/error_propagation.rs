use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("username.txt");
  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }

  // o de forma menos verbosa
  // let mut s2 = String::new();
  // f.read_to_string(&mut s2)?; // el operador `?` define *casi* el mismo comportamiento que el match anterior.
  // Ok(s2);                     // Al usar `?`, se llama a la función `from` sobre el error para convertirla al
                                 // mismo tipo que el que devuelve la función, especificado en Result<>
                                 // `?` solo puede usarse en funciones que devuelvan Result<>

  // También es posible hacer encadenado de funciones usando `?`:
  // let mut s3 = String::new();
  // File::open("username.txt")?.read_to_string(&mut s3)?;
  // Ok(s3)
}

fn main() {
  match read_username_from_file() {
    Ok(username) => println!("The username is: {:?}", username),
    Err(e) => panic!("Could not read username: {:?}", e),
  };
}
