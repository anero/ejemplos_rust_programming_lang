fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // `move`, invalida `s1`, así que si se usa `s1` a partir de acá hay un error de compilación
    // println!("s1: {}, s2: {}", s1, s2); error

    let s1 = s2.clone(); // hace deep copy y no invalida `s21
    println!("s1: {}, s2: {}", s1, s2);

    let x = 5;
    let y = x; // los tipos nativos (que implementan el trait Copy) se guardan en stack así que no hay necesidad de hacer `clone`
    println!("x: {} y: {}", x, y);

    // Los parámetros de funciones también son movidos al pasarse:
    let s = String::from("hello 2");
    takes_ownership(s); // `s` fue "movido" así que no se puede volver a usar la variable en este scope
    // println!("{}", s); // el compilador tira error

    let x = 5;
    makes_copy(x); // es un tipo nativo así que no es "movido"

    let s1 = gives_ownership();   // la función da el ownership de la variable `s1` al retornarla
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // da el ownership al argumento y recibe el ownership de `s3` que devolvió la funci´øn

    // Pero se puede pasar el parámetro por referencia, lo que evita que sea "movida"
    let s4 = String::from("hello");
    let length = calculate_length(&s4); // ademas de declarar el parámetro por referencia hay que pasar el argumento por referencia
    println!("String: {} length: {}", s4, length); // Puedo seguir usando la variable `s4` porque no se transifió el ownership

    let mut s5 = String::from("hello"); // Hay que declarar la variable mutable para poder pasarla así a la función
    change(&mut s5); // ademas de pasar el argumento por parametro hay que declarar que es mutable
    println!("{}", s5);

    let r1 = &mut s5;
    // let r2 = &mut s5; // no se puede tener más de una referencia mutable a una variable en un scope

    let mut s6 = String::from("hello");
    let r1 = &s6; // no hay problema porque es una referencia inmutable
    let r2 = &s6; // no hay problema porque es una referencia inmutable
    // let r3 = &mut s6; // no compila porque ya existen referencias a la variable (inmutables)

    // Rust se encarga de no crear referencias a variables que hayan sido liberadas
    // let reference_to_nothing = dangle();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello desde gives_ownership");
    some_string   // Creo una nueva variable y cede el ownership a quien la invocó
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // Devuelve la misma variable que recibió, así que devuelve el ownership
}

fn calculate_length(s: &String) -> usize { // `s` está declarada por referencia (la función hace "borrowing" de la variable)
    // No se pueden modificar las variables que son recibidas por parámetro
    // s.push_str(" world"); // Tira error de compilación
    s.len()
}

fn change(s: &mut String) {
  s.push_str(" world"); // Al especificar que es mutable con `mut` se puede modificar
}

// fn dangle() -> &String {
//   let s = String::from("hello");
//   &s // el compilador tira error
// }