// iterators.rs

fn main() {
    {
        // Processing a Series of Items with Iterators
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    {
        // Methods that Produce Other Iterators
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
        println!("Got: {:?}", v2);
    }

    {
        // Using Closures that Capture Their Environment
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }

        fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }

        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);
        println!("{:?}", in_my_size);
    }

    {
        // Creating Our Own Iterators with the Iterator Trait
        #[derive(Debug)]
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let mut counter = Counter::new();
        for _ in 0..5 {
            counter.next();
            println!("{:?}", counter);
        }

        // Using Other Iterator Trait Methods
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("{}", sum);
    }
}
