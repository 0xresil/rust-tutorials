




struct Point {
    x: f32,
    y: f32
}

fn match_tuple(t: (i32, String)) {
    let x = match t {
        (0, s) => format!("zero {}", s),
        (1, s) if s == "hello" => format!("hello one"),
        no_match => format!("No match {:?}", no_match)
    };
    println!("{}", x);
}

fn main() {
    let t = (10, "hello".to_string());
    let (ref n, ref s) = t;
    println!("n is {}", n);
    println!("s is {}", s);

    let p = Point { x: 1.0, y: 2.0 };
    let Point {x, y} = p;
    println!("{} {}", x, y);

    match_tuple((1, "wasdf".to_string()));

    let ot = Some((2, "hello".to_string()));
    if let Some((_, ref s)) = ot {
        println!("{}", s);
    }
}