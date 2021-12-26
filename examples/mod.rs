
use std::str;

mod foo {
    #[derive(Debug)]
    pub struct Foo {
        pub s: &'static str
    }

    impl Foo {
        pub fn new(s: &'static str) -> Foo {
            Foo { s: s }
        }
    }
}

fn somefn() -> Option<u32> {
    Some(42)
}

fn main() {
    let f = foo::Foo::new("hello");

    let x = foo::Foo { s: "asdf" } ;
    println!("{:?} {:?}", f, x);

    let s = String::from(r#"{"asdf"}gesfd"#);
    println!("{}", s);

    let chars: Vec<char> = s.chars().collect::<Vec<_>>();
    let bytes: Vec<u8> = s.as_bytes().to_vec();

    let utfs = str::from_utf8(&bytes).unwrap();

    let chars2: Vec<char> = bytes.iter().map(|x| *x as char).collect::<Vec<_>>();
    println!("{:?}", chars2);

    // variable binding
    let number = 5;
    match number {
        1 => println!("one"),
        n@(2 | 3 | 5 | 8) => println!("{}", n),
        n@13..=19 => println!("tenn {}", n),
        _ => println!("no")
    }

    let refvar = "asdgia;lkjsdf";
    match refvar {
        val => println!("{:?}", refvar)
    }

    match somefn() {

    }

    println!()
}