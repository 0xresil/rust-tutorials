
use std::fmt;

struct Person {
    first_name: String,
    last_name: String
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I am {}!", self.full_name())
    }
}

impl Person {
    fn new(first: &str, last: &str) -> Self {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn copy(&self) -> Self {
        Self::new(&self.first_name, &self.last_name)
    }

    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }
    
    fn to_touple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
fn main() {
    let mut p = Person::new("john", "max");
    println!("{:?}", p);
    println!("{} {}", p.first_name, p.last_name);

    let c = p.copy();
    println!("c is {:?}", c);
    p.first_name = "asdfasdf".to_string();
    println!("c is {:?}", c);
    println!("p is {:?}", p);


    let x = "asdfasdf";
    let y = x.to_string();
    println!("x is {}", x);
    
    
    println!("{:?}", p.to_touple());
}

