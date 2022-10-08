fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest(&char_list);
  println!("The largest char is {}", result);

  // Uso de generics en struct
  struct Point<T> {
    x: T,
    y: T
  }
  let both_integer = Point { x: 5, y: 10 };
  let both_float = Point { x: 1.0, y: 4.0 };

  // Se pueden definir multiples tipos
  struct Point2<T, U> {
    x: T,
    y: U
  }
  let both_integer = Point2 { x: 5, y: 10 };
  let both_float = Point2 { x: 1.0, y: 4.0 };
  let integer_and_float = Point2 { x: 5, y: 4.0 };

  // Uso de generics en enum (clon de Option<T>)
  enum Opcional<T> {
    Alguno(T),
    Ninguno,
  }

  // Uso de generics en definición de métodos de structs / enums
  impl<T> Point<T> {
    fn x(&self) -> &T {
      &self.x
    }
  }

  // Implementación de método para un tipo específico
  impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }

  // El método puede usar generics que no son los del struct
  impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
      Point2 {
        x: self.x,
        y: other.y,
      }
    }
  }
  let p1 = Point2 { x: 5, y: 10.4 };
  let p2 = Point2 { x: "Hello", y: 'c' };
  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
