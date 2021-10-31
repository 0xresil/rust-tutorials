
struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {val: x1, end: x2, incr: skip}
}

impl Iterator for FRange {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res > self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}

fn dump<T> (value: &T) 
where T: std::fmt::Debug {
    println!("value is {:?}", value);
}

fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul + Copy {
    x * x
}

#[derive(Debug)]
struct Foo {
    name: String
}

fn main() {
    for x in range(0.0, 1.0, 0.1) {
        println!("{:1.1}", x);
    }

    let v: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.sin()).collect();
    println!("v is {:?}", v);

    let x = 10;
    dump(&x);
    let y = 10.0345;
    dump(&y);

    let foo = Foo { name: "hello".to_string() };
    dump(&foo);

    println!("{}", sqr(10));
}