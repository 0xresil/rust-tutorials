
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
}