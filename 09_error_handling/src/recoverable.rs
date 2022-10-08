use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("hello.txt"); // open() devuelve un Result<File, std::io::Error>
                                   // Si open() puede abrir el archivo, f será de tipo File
                                   // Si encuentra un error, f será de tipo std::io::Error
  let _f = match f {
    Ok(file) => file,
    Err(ref error) if error.kind() == ErrorKind::NotFound => {
        match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => {
            panic!("Tried to create file but there was a problem: {:?}", e)
          },
        }
    },
    Err(error) => {
      panic!("There was a problem opening the file {:?}", error)
    },
  };

  // Otra forma menos verbosa
  // let _f2 = File::open("hello2.txt").unwrap(); // unwrap() devuelve la referencia a File o llama a panic!()
  // let _f3 = File::open("hello2.txt").expect("Failed to open hello2.txt"); // expect() funciona igual que unwrap(), pero permite definir el mensaje de error que se le pasa a panic!()
}
