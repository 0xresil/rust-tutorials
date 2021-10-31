
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

    // if let sentences

    let ot = Some((2, "hello".to_string()));
    if let Some((1, ref s)) = ot {
        println!("{}", s);
    } else {
        println!("no match");
    }

    if let Ok(n) = "42".parse::<i32>() {
        println!("n is {}", n);
    }

    let n: i32 = "44".parse().unwrap();
    println!("n is {}", n);
}