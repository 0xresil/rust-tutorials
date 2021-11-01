
#[derive(Debug)]
struct Fivo {
    first: u32,
    second: u32
}

impl Iterator for Fivo {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.first + self.second;
        self.first = self.second;
        self.second = res;
        
        if res > 1000 {
            None
        } else {
            Some(res)
        }
    }
}

#[derive(Debug)]
struct StrFivo {
    first: String,
    second: String
}

impl Iterator for StrFivo {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.first.to_string() + &self.second;
        self.first = self.second.to_string();
        self.second = res.to_string();
        if res.len() > 200 {
            None
        } else {
            Some(res.to_string())
        }
    }
}

fn main() {
    let fivonachi_iter = || Fivo { first: 1, second: 2};
    for i in fivonachi_iter() {
        println!("{}", i);
    }

    let fivostr_iter = |a: &str, b: &str| StrFivo { first: a.to_string(), second: b.to_string() } ;
    for s in fivostr_iter("he", "wo") {
        println!("{}", s);
    }

    let fivo_serial: Vec<u32> = fivonachi_iter().collect();
    println!("{:?}", fivo_serial);

    let fivo_serial1: Vec<u32> = fivonachi_iter().filter(|x| x % 2 == 1).collect();
    println!("{:?}", fivo_serial1);

    let tuples = [(10, "ten"), (20, "twenty"), (30, "thirty"), (40, "forty")];
    let iter = tuples.iter().filter(|t| t.0 > 20).map(|t| t.0);

    for name in iter {
        println!("{:?} ", name);
    }

    let fivo_str_serial: Vec<String> = fivostr_iter("hi","hello").collect();
    for n in fivo_str_serial.iter().map(|x: &String| x.len()) {
        println!("{:?}", n);
    }

    for n in fivo_str_serial.iter().filter(|x| *x == "hello") {
        println!("{:?}", n);
    }
    println!("{:?} ", fivo_str_serial);

}