// Si declaro la función así, el borrow checker no sabe si la referencia que se devuelve fue prestada de `x` o de `y`
// fn longest(x: &str, y: &str) -> &str {

// Para declararlo explicitamente, se define un **generic lifetime parameter**
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {   // Esta declaración le dice al compilador que la vida de las referencias `x` e `y`, y de
                                                      // la referencia que devuelve tiene que ser tan larga como la de la vida genérica `'a`.
                                                      // Esto **NO** cambia la vida de las variables que se pasan como parámetro, solamente
                                                      // establece condiciones para que en caso de no cumplirse se arroje un error en tiempo de
                                                      // compilación
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn first<'a>(x: &'a str, y: &str) -> &'a str {  // Como siempre se devuelve una referencia al primer parámetro, no hace falta explicitar
                                                // la vida del segundo parámetro.
                                                // La vida de una referencia devuelta por una función **SIEMPRE** tiene que ser la misma que al
                                                // menos uno de los parámetros. De otra forma estaría devolviendo una referencia a una variable
                                                // interna de la función, que es inválida ni bien se sale del scope de la función.
  x
}

// Se puede definir un struct que tiene referencias, pero hay que definir explícitamente todos la vida en la definición
struct ImportantExcerpt<'a> {
  part: &'a str,
}

// Lifetime elision rules:
// Reglas que aplica el compilador para intentar inferir las vidas y evitar que se definan explícitamente.
// 1- Cada parámetro de entrada que es una referencia tiene su propia vida. ej: fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
// 2- Si hay un solo parámetro, su vida será asignada a todos los parámetros de salida. ej: fn <'a>(x: &'a i32) -> &'a i32;
// 3- Si hay varios parámetros, pero uno de ellos es `&self` o `mut &self` porque es un método, entonces se asigna la vida de
//    `self` a todos los parámetros de salida

impl <'a> ImportantExcerpt<'a> {
  // El método no está atado a la vida del campo del struct. No hace falta anotar la vida
  // del parámetro `self` por la primer regla de elision
  fn level(&self) -> i32 {
    3
  }

  // Tampoco hace falta anotar la vida en este método porque se aplica primero la 1ra y después la 3er regla de elision:
  // Les asigna a `self` y `announcement` sus propias vidas, y luego como está el parámetro `self` le aplica la vida a la referencia que retorna el método
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

fn main() {
  // La variable `r` tiene una vida más larga que `x`, por lo que el código no compila
  // de otra forma al salir del bloque de código interno, `r` quedaría apuntando a memoria ya liberada
  // let r;
  // {
  //   let x = 5;
  //   r = &x;
  // }
  // println!("r: {}", r);

  let string1 = String::from("abcd");
  let string2 = "xyz";

  // Al pasar referencias concretas a la función, la vida concreta que sustituye a `'a` es la parte del
  // scope de `x` que coincide con el scope de `y`, o sea `'a` toma el valor que sea más chico entre
  // las vidas de `x` e `y`. La referencia que devuelve la función también tiene esa vida
  let result = longest(string1.as_str(), string2);

  println!("The longest string is {}", result);

  // Otro ejemplo con vidas distintas para las referencias
  let string3 = String::from("long string is long");
  {
    let string4 = String::from("xyz");
    // `'a` toma el valor de la vida de `string4`
    let result = longest(string3.as_str(), string4.as_str());
    println!("The longest string from within the block is {}", result);
  }

  // Este ejemplo no funciona
  let string5 = String::from("long string is long");
  let result;
  {
    let string6 = String::from("xyz");
    // `'a` toma el valor de vida de `string6`
    result = longest(string5.as_str(), string6.as_str());
  }
  // `result` es inválido acá porque string6 ya está fuera de scope
  // println!("The longest string from within the block is {}", result);

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentece = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt { part: first_sentece };

  // Se puede usar la vida especial `'static`, que implica que la referencia es válida durante toda la ejecución del programa. Todos los
  // strings literales tienen esta vida:
  let s: &'static str = "I have a static lifetime";   // El texto del string se guarda en el binario del programa y siempre está disponible
}