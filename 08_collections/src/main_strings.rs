fn main() {
  let mut s = String::new(); // string vacio y mutable

  // Strings inmutables y con contenido ("literales")
  let data = "initial contents";
  let s = data.to_string();
  let s = "initial contents".to_string();
  let s = String::from("initial contents");


  //// Modificando el string
  let mut s = String::from("foo");
  s.push_str("bar");

  let s2 = "bar";
  s.push_str(s2); // push_str no toma ownership del parametro
  println!("s2 es {}", s2);

  {
    let s1 = String::from("Hola, ");
    let s2 = String::from("mundo!");
    let s3 = s1 + &s2;  // s1 ya no se puede usar aca porque el metodo `+` toma ownership del self
    println!("s3 es {}", s3);
  }

  {
    let s1 = String::from("Hola");
    let s2 = String::from("mundo!");
    let s = format!("{}, {}", s1, s2); // s1 y s2 se pueden seguir usando porque la macro format! no toma ownership
    println!("s formateado es {}", s);
  }

  //// Iterando sobre los caracteres
  {
    for c in "Hola".chars() {  // `chars()` devuelve los elementos como caracteres UTF-8
      println!("{}", c);
    }
  }

  {
    for c in "Hola".bytes() {  // `bytes()` devuelve los elementos como byte
      println!("{}", c);
    }
  }
}
