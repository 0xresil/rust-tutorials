
#[derive(Debug, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
use Direction::*;

impl Direction {
    fn as_str(&self) -> &'static str {
        match self {
            Up => "up",
            Right => "right",
            Down => "down",
            Left => "left"
        }
    }
    fn next(&self) -> Self {
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}

#[derive(Debug)]
enum ValueEnum {
    Number(f64),
    Str(String),
    Bool(bool)
}

use ValueEnum::*;

impl<'a> ValueEnum {
    fn to_str(&'a self) -> Option<&'a String> {
        match self {
            Str(s) => Some(s),
            _ => None            
        }
    }
    fn to_str1(&self) -> Option<&String> {
        if let Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

fn eat_and_dump(x: ValueEnum) {
    use ValueEnum::*;
    match x {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is {}", s),
        Bool(b) => println!("bool is {}", b)
    }
}

fn dump(x: &ValueEnum) {
    match *x {
        Number(n) => println!("number is {}", n),
        Str(ref s) => println!("string is {}", s),
        Bool(b) => println!("bool is {}", b)
    }
}

fn main() {
    let mut x = Direction::Left;
    assert_eq!(x, Direction::Left);
    println!("x as str is {}", x.as_str());

    for _ in 0..8 {
        println!("x {:?} value is {}", x.next(), x.next() as u32);
        if x.next() > Direction::Left {
            println!("bigger than Left");
        }   
        x = x.next();        
    }
    
    use ValueEnum::*;
    let n = ValueEnum::Number(2.4);
    let s = Str("hello".to_string());
    let b = Bool(true);
    println!("{:?} {:?} {:?}", n, s, b);
    println!("{:?} {:?} {:?}", n, s, b);
    dump(&n);
    dump(&s);
    dump(&b);

    println!("s? {:?}", s.to_str());

    if n.to_str1() == None {
        println!("to str is None");
    } else {
        println!("s {:?}", n.to_str1());
    }

    eat_and_dump(n);
    eat_and_dump(s);
    eat_and_dump(b);    

    let dirx = Direction::Up;
    if Direction::Up == dirx {
        println!("direction is up");
    }
}