
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
}