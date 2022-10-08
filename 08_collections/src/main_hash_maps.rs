use std::collections::HashMap;

fn main() {
  // Constructor de un hash map vacio
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Red"), 50);

  println!("scores: {:?}", scores);

  // Constructor inicializando el hash map
  let teams = vec![String::from("Blue"), String::from("Red")];
  let initial_scores = vec![10, 50];

  // Hay que especificar el tipo de scores2 porque collect() devuelve muchos tipos de estructuras
  // de datos distintas. No hace falta especificar el tipo de los pares key/value porque se
  // infieren automaticamente
  let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

  println!("scores2: {:?}", scores2);

  // Ownership:
  // Si el tipo implementa el trait Copy, el valor se copia al hash map
  // Si el tipo es "owned", el valor se mueve y el hash map pasa a ser el dueño
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);  // `field_name` y `field_value` ya no pueden usarse porque `map` es ahora el dueño

  // Obtener valores del HashMap
  let team_name = String::from("Blue");
  let blue_score = scores.get(&team_name);
  println!("Blue score: {:?}", blue_score); // Devuelve el resultado con Some porque la función devuelve Option<&V>.
                                            // Si la key no está en el hash map, get() devuelve None

  for (key, value) in &scores {       // Se recorre el hash map en orden arbitrario
    println!("{}: {}", key, value);
  }

  // Actualizar un hash map

  // Sobreescribiendo el valor
  scores.insert(String::from("Blue"), 100);
  println!("Blue score: {:?}", scores.get(&String::from("Blue")));

  // Chequeando que no exista antes de insertarlo
  scores.entry(String::from("Yellow")).or_insert(50); // entry() devuelve un enum de tipo Entry
  scores.entry(String::from("Blue")).or_insert(999);
  println!("{:?}", scores);

  // Actualizando la key basado en el anterior valor
  let text = "hello world wonderful world";
  let mut text_map = HashMap::new();
  for word in text.split_whitespace() {
    let count = text_map.entry(word).or_insert(0);  // entry() y or_insert() devuelven una referencia mutable al valor si ya existía
    *count += 1;                                    // Se mantiene una referencia mutable en la variable `count`, así que para asignarle un valor
                                                    // ha que dereferenciarla usando el `*`
  }
  println!("{:?}", text_map);

}
