// pointers.rs

// In Rust, which uses the concept of ownership and borrowing, an additional difference between
// references and smart pointers is that references are pointers that only borrow data;
// in contrast, in many cases, smart pointers own the data they point to.

// The characteristic that distinguishes a smart pointer from an ordinary struct is that
// smart pointers implement the Deref and Drop traits. The Deref trait allows an instance of
// the smart pointer struct to behave like a reference so you can write code that works with
// either references or smart pointers. The Drop trait allows you to customize the code that
// is run when an instance of the smart pointer goes out of scope.

// Given that the smart pointer pattern is a general design pattern used frequently in Rust.
// Many libraries have their own smart pointers, and you can even write your own.
// Most common smart pointers in the standard library:
// - Box<T> for allocating values on the heap
// - Rc<T>, a reference counting type that enables multiple ownership
// - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

fn main() {
    {
        // Using Box<T> to Point to Data on the Heap
        // Boxes allow you to store data on the heap rather than the stack.

        let b = Box::new(5);
        println!("b = {}", b);

        enum List {
            // Cons(i32, List), // recursive construction unknown memory consumption
            Cons(i32, Box<List>),
            Nil,
        }
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    }

    {
        // Treating Smart Pointers Like Regular References with the Deref Trait
        // Implementing the Deref trait allows you to customize the behavior of the dereference operator *
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        // Defining Smart Pointer
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> std::ops::Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        // Implicit Deref Coercions with Functions and Methods
        let hello = |name: &str| println!("Hello, {}!", name);
        let m = MyBox::new(String::from("Rust"));

        (hello)(&m);
        // Here we’re calling the hello function with the argument &m, which is a reference to a
        // MyBox<String> value. Because we implemented the Deref trait on MyBox<T>,
        // Rust can turn &MyBox<String> into &String by calling deref.

        (hello)(&(*m)[..]);
        // The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take a string
        // slice of the String that is equal to the whole string to match the signature of hello.

        // How Deref Coercion Interacts with Mutability
        // Similar to how you use the Deref trait to override the * operator on immutable references,
        // you can use the DerefMut trait to override the * operator on mutable references.
    }

    {
        // Running Code on Cleanup with the Drop Trait
        struct CustomSmartPointer {
            data: String,
        }

        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data);
            }
        }

        {
            let c = CustomSmartPointer {
                data: String::from("my stuff"),
            };
            let d = CustomSmartPointer {
                data: String::from("other stuff"),
            };
            println!("CustomSmartPointers created.");
        }

        {
            let c = CustomSmartPointer {
                data: String::from("some data"),
            };
            println!("CustomSmartPointer created.");
            drop(c);
            println!("CustomSmartPointer dropped before the end of main.");
        }
    }

    {
        // Rc<T>, the Reference Counted Smart Pointer
        enum List {
            Cons(i32, std::rc::Rc<List>),
            Nil,
        }

        let a = std::rc::Rc::new(List::Cons(5, std::rc::Rc::new(List::Cons(10, std::rc::Rc::new(List::Nil)))));
        println!("count after creating a = {}", std::rc::Rc::strong_count(&a));
        let b = List::Cons(3, std::rc::Rc::clone(&a));
        println!("count after creating b = {}", std::rc::Rc::strong_count(&a));
        {
            let c = List::Cons(4, std::rc::Rc::clone(&a));
            println!("count after creating c = {}", std::rc::Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", std::rc::Rc::strong_count(&a));
    }

    {
        // RefCell<T> and the Interior Mutability Pattern
        // Interior mutability is a design pattern in Rust that allows you to mutate data even when
        // there are immutable references to that data; normally, this action is disallowed by the borrowing rules.

        // Enforcing Borrowing Rules at Runtime with RefCell<T>
        // Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.
        // - At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
        // - References must always be valid.
        // With RefCell<T>, if you break these rules, your program will panic and exit.

        // Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
        // - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
        // - Box<T> allows immutable or mutable borrows checked at compile time;
        // Rc<T> allows only immutable borrows checked at compile time;
        // RefCell<T> allows immutable or mutable borrows checked at runtime.
        // - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
        // inside the RefCell<T> even when the RefCell<T> is immutable.

        // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
        #[derive(Debug)]
        enum List {
            Cons(std::rc::Rc<std::cell::RefCell<i32>>, std::rc::Rc<List>),
            Nil,
        }

        let value = std::rc::Rc::new(std::cell::RefCell::new(5));

        let a = std::rc::Rc::new(List::Cons(std::rc::Rc::clone(&value), std::rc::Rc::new(List::Nil)));

        let b = List::Cons(std::rc::Rc::new(std::cell::RefCell::new(3)), std::rc::Rc::clone(&a));
        let c = List::Cons(std::rc::Rc::new(std::cell::RefCell::new(4)), std::rc::Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    {
        // Reference Cycles Can Leak Memory

        // Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug)]
        struct Node {
            value: i32,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });

        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    }

    {
        // Adding a Reference from a Child to Its Parent
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        // Visualizing Changes to strong_count and weak_count
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
