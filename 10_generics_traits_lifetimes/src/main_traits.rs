use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
  // Sin implementación por defecto:
  // fn summarize(&self) -> String;

  // Con implementación por defecto:
  fn summarize(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author())
  }

  fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  // para usar la implementación por defecto, se deja vacío el bloque `impl`

  // Haciendo override de la implementación por defecto
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }

  fn summarize_author(&self) -> String {
    format!("{}", self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  // fn summarize(&self) -> String {
  //   format!("{}: {}", self.username, self.content)
  // }

  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

// Especificación de un generic type que debe implementar cierto trait -- **trait bound**
pub fn notify<T: Summary>(item: T) {
  println!("Breaking news! {}", item.summarize());
}

// Para definir múltiples trait bounds sobre un generic type se usa `+`
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
  // ...
  10
}
// O de forma más suscinta:
pub fn some_function2<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
{
  // ..
  10
}

// Implementación condicional de métodos basados en trait bounds
struct Pair<T> {
  x: T,
  y: T,
}
impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    x,
    y,
  }
}
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

// Implementación condicional de métodos para cualquier trait que implemente otro trait -- **blanket implementation**
impl<T: Display> ToString for T {
  // Cualquier tipo T que implemente el trait `Display` va a recibir los métodos que se declaren en este bloque
}

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());

  let news_article = NewsArticle {
    headline: String::from("You are not going to believe this"),
    location: String::from("Frontpage"),
    author: String::from("Clark Kent"),
    content: String::from("This is amazing, read on to learn!"),
  };

  println!("1 new article: {}", news_article.summarize());

  notify(tweet);
  notify(news_article);
}
