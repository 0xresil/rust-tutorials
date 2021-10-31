
fn apply<F>(x: f64, f: F) -> f64 
where F: Fn(f64) -> f64 {
    f(x)
}

fn mutate<F> (mut f: F) 
where F: FnMut() {
    f()
}

fn main() {
    let f = |x| x*x;
    let res = f(10);
    println!("res {}", f(1));

    let a = 2.0;
    let b = 1.0;
    let f1 = |x| a*x + b;
    println!("res is {}", f1(2.0));
    
    let res1 = apply(3.0, f1);
    let res2 = apply(3.14, |x| x.sin());
    println!("res1 {} res2 {}", res1, res2);

    let mut s = "world";
    mutate(|| s = "hello");
    println!("s is {}", s);
    assert_eq!(s, "hello");

    let mut s = "world";
    let mut changer = || s = "hello";
    changer();

    assert_eq!(s, "hello");
    println!("{}", s);

    let name = "dolly".to_string();
    let age = 42;

    let c_name = name.clone();
    let c = move || {
        println!("name is {} age is {}", c_name, age);
    };
    c();

    println!("{}", name);
}