// structs.rs

// Defining Structs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Using the Field Init Shorthand when Variables and Fields Have the Same Name
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Methods
impl User {
    // constructor
    fn new(email: String, username: String, sign_in_count: u64, active: bool) -> Self {
        Self {
            email,
            username,
            sign_in_count,
            active,
        }
    }

    // method
    fn to_string(&self) -> String {
        format!("email {}, username {}, sign in count {}, active {}", self.email, self.username, self.sign_in_count, self.active)
    }
}

fn main() {
    // Instantiating Structs
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user name: {}", user1.username);

    // Using the Field Init Shorthand when Variables and Fields Have the Same Name
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    user1.active = false;

    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("user name: {}", user2.username);

    // Methods
    let user3 = User::new(String::from("next@example.com"), String::from("nextusername89"), 5, true);
    println!("{}", user3.to_string().as_str());
}
