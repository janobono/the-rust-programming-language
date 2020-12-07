// traits.rs
// Traits are similar to a feature often called interfaces in other languages.

// Defining a Trait
pub trait BookSummary {
    fn author_summarize(&self) -> String;

    fn publisher_summary(&self) -> String {
        String::from("Our Small Company Inc.")
    }

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.author_summarize())
    }
}

// Implementing a Trait on a Type
#[derive(Debug)]
pub struct Book {
    pub author: String,
    pub info: String,
}

impl BookSummary for Book {
    fn author_summarize(&self) -> String {
        format!("author : {}", self.author)
    }
}

impl BookSummary for i32 {
    fn author_summarize(&self) -> String {
        format!("i32 : {}", self)
    }
}

// Traits as Parameters
fn notify(item: &impl BookSummary) {
    println!("Summary: {}", item.summarize());
}

// Trait Bound Syntax - if there is more than one parameter of this type
pub fn notify2<T: BookSummary>(item: &T) {
    println!("Summary2: {}", item.summarize());
}

// Clearer Trait Bounds with where Clauses
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

}

fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

}

fn main() {
    let book = Book { author: String::from("Johny Pytlik"), info: String::from("Super book") };
    println!("{:?}", book);
    println!("{}", book.summarize());

    let v: i32 = 50;
    println!("{}", v.summarize());

    notify(&v);
    notify2(&v);
}
