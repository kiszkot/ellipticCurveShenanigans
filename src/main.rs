use rug::Integer;
use elliptic::{Curve, Point, degree};

fn main() {
    println!("Hello, world!");
    let curve: Curve = Curve {
        a: Integer::from(497),
        b: Integer::from(1768),
        p: Integer::from(9739),
    };
    let x: Point = Point {
        x: Some(Integer::from(5274)), 
        y: Some(Integer::from(2841)),
        curve: Some(curve.clone()),
    };
    let y: Point = Point {
        x: Some(Integer::from(8669)),
        y: Some(Integer::from(740)),
        curve: Some(curve.clone()),
    };
    let x_x = x.clone() + x.clone();
    let x_y = x.clone() + y.clone();
    println!("X + X = {}", x_x);
    println!("X + Y = {}", x_y);

    let mut p: Point = Point {
        x: Some(Integer::from(493)),
        y: Some(Integer::from(5564)),
        curve: Some(curve.clone()),
    };
    let q: Point = Point {
        x: Some(Integer::from(1539)),
        y: Some(Integer::from(4742)),
        curve: Some(curve.clone()),
    }; 
    let r: Point = Point {
        x: Some(Integer::from(4403)),
        y: Some(Integer::from(5202)),
        curve: Some(curve.clone()),
    };
    let mut s = p.clone() + p + q + r;
    println!("s = {}", s);

    let t: Point = Point {
        x: Some(Integer::from(5323)),
        y: Some(Integer::from(5438)),
        curve: Some(curve.clone()),
    };
    s = t.clone() * Integer::from(1337);
    println!("s = {}", s);

    let tst = x * Integer::from(2);
    println!("{}", tst);

    p = Point {
        x: Some(Integer::from(2339)),
        y: Some(Integer::from(2213)),
        curve: Some(curve.clone()),
    };
    s = p * Integer::from(7863);
    println!("s = {}", s);

    //let g: Point = Point { x: Some(Integer::from(1804)), y: Some(Integer::from(5368)), curve: Some(curve.clone()) };
    let n = Integer::from(1829);
    p = Point {
        x: Some(Integer::from(815)),
        y: Some(Integer::from(3190)),
        curve: Some(curve.clone()),
    };
    s = p * n;
    println!("shared secret: {}", s);

    // Mati
    let mati: Curve = Curve {
        a: Integer::from(7),
        b: Integer::from(3),
        p: Integer::from(13),
    };
    let g: Point = Point {
        x: Some(Integer::from(3)),
        y: Some(Integer::from(5)),
        curve: Some(mati.clone()),
    };

    let tst = degree(&g);
    println!("{}", tst);

    for i in 1..14 {
        let t = g.clone() * Integer::from(i);
        println!("{}: {}", i, t);
    }
}

// 10 + 3 = 0 mod 13
// 10 = -3 mod 13
// (a, 10)
// (a, 3)
// -10 = 3 mod 13
