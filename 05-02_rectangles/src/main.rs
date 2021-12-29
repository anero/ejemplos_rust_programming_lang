#[derive(Debug)] // Annotation para hacer derive del trait `Debug`
struct Rectangle {
    height: u32,
    width: u32
}

// `impl` define un "implementation block"
impl Rectangle {
    fn area(&self) -> u32 {       // el primer parámetro siempre tiene que se `self`. Pueden usarse `&` y `mut` como con otros parámetros
        self.height * self.width
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    fn square(size: u32) -> Rectangle {         // "associated function", no es un método porque no recibe el `self`
      Rectangle { height: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { height: 50, width: 30};

    println!("{:?}", rect1); // `{:?}` usa el output format debugger
    println!("{:#?}", rect1); // `{:#?}` es otro ejemplo de output format

    println!(
        "The rectangle area is {}",
        rect1.area()             // No es necesario usar diferentes operadores dependiendo si la variable es una refencia o no como en C y C++ (`->` vs. `.`).
    );                          // En vez de eso, Rust usa "automatic referencing and dereferencing", que internamente convierte
                                // automáticamente `p1.distance(&p2)` en `(&p1).distance(&p2)`

    let rect2 = Rectangle { height: 40, width: 10 };
    let rect3 = Rectangle { height: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(20);
    println!("{:?}", rect4);

}
