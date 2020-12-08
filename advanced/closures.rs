// closures.rs

fn main() {
    {
        fn add_one_v1(x: u32) -> u32 { x + 1 }
        println! {"add_one_v1({}) = {}", 10, add_one_v1(10)};

        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        println! {"add_one_v2({}) = {}", 10, add_one_v2(10)};
    }

    {
        // Storing Closures Using Generic Parameters and the Fn Traits
        let mut catcher = Cacher::new(|x: u32| -> u32 { x + 1 });
        println! {"{}", catcher.value(10)}
        println! {"{:?}", catcher.value}
    }

    {
        // Capturing the Environment with Closures
        let x = 4;
        let equal_to_x = |z| {z == x};
        let y = 4;
        assert!(equal_to_x(y));
    }
}

struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
