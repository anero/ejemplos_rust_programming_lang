struct User {
    username: String, // no definimos los atributos como `&str` porque queremos que el struct tenga ownership sobre la variable
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    // instanciacion del struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    // la variable struct tiene que ser `mut` para poder modificar cualquiera de sus atributos, no se puede hacer solo uno de los atributos mutable
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    user2.username = String::from("otro_username");

    let user3 = build_user(String::from("someone2@example.com"), String::from("some_other_user"));

    // Inicialización usando otra instancia del mismo struct con "struct update syntax"
    let user4 = User {
        email: String::from("someone3@example.com"),
        username: String::from("yet_another_user"),
        ..user3
    };

    // "tuple structs" son structs con atributos anónimos
    struct Point(i32, i32, i32);
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    // Instanciación usando "field init shorthand"
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}