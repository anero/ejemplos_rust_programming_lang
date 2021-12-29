fn main() {
  //// Inmutables

  // Sin valores ni inferencia de tipo
  let v: Vec<i32> = Vec::new();

  // Inicializado con valores e inferencia de tipo
  let v2 = vec![1, 2, 3];

  //// Mutables

  // Infiere el tipo a partir del primer elemento que se agrega
  let mut v = Vec::new();
  v.push(5);
  v.push(4);
  v.push(3);
  v.push(2);
  v.push(1);

  //// Liberacion de memoria

  {
    let v = vec![1,2,3];
    // hago algo con v
  }
  // al salir del bloque se libera la memoria del vector y todos sus elementos


  //// Referenica a los elementos
  {
    let v = vec![1, 2, 3, 4, 5];

    // Ambas formas son validas
    let third: &i32 = &v[2];  // panic si el elemento no existe
    let third2: Option<&i32> = v.get(2); // devuelve `None` sin panic


    let mut v_mut = vec![1, 2, 3, 4];
    let first = &v[0];
    // v.push(6); <- No compila porque v_mut es inmutable por la referencia al primer elemento inmutable
  }


  //// Iterando sobre los elementos
  {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
      *i += 50;
    }

    println!("v: {:?}", v); // v: [150, 82, 107]
  }


  //// Almacenando elementos de diferentes tipos
  {
    enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String)
    }

    let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Float(10.12),
      SpreadsheetCell::Text(String::from("blue"))
    ];
  }
}
