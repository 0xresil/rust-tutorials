

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let pi = 3.14;
    println!("{}", answer.show());
    println!("{}", pi.show());
}
