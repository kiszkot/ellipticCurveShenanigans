use rug::Integer;

#[derive(PartialEq, Clone)]
pub struct Curve {
    pub a: Integer,
    pub b: Integer,
    pub p: Integer,
}

impl std::fmt::Display for Curve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "y^2 = x^3 + {} x + {} (mod {})", self.a, self.b, self.p)
    }
}

#[derive(PartialEq, Clone)]
pub struct Point {
    pub x: Option<Integer>,
    pub y: Option<Integer>,
    pub curve: Option<Curve>,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.x.is_none() || self.y.is_none() {
            write!(f, "(None, None)")
        } else {
            let x = self.x.as_ref().unwrap();
            let y = self.y.as_ref().unwrap();
            write!(f, "({}, {})", x, y)
        }
    }
}

pub fn euclides_gmp(a: &Integer, b: &Integer) -> (Integer, Integer, Integer) {
    let mut x1: Integer = Integer::from(1);
    let mut y1: Integer = Integer::from(0);
    let mut x2: Integer = Integer::from(0);
    let mut y2: Integer = Integer::from(1);
    let mut q: Integer;
    let mut r1: Integer = a.clone();
    let mut r2: Integer = b.clone();
    while r2 != 0 {
        (r1, (q, r2)) = (r2.clone(), r1.div_rem(r2));
        (x1, x2) = (x2.clone(), x1 - x2 * &q);
        (y1, y2) = (y2.clone(), y1 - y2 * &q);
    }
    return (r1, x1, y1);
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _q: Point) -> Point {
        let o: Point = Point {
            x: None,
            y: None,
            curve: self.curve.clone(),
        };
        let mut ret: Point = Point {
            x: Some(Integer::from(0)),
            y: Some(Integer::from(0)),
            curve: self.curve.clone(),
        };
        if self == o {
            return _q.clone();
        }
        if _q == o { 
            return self;
        }

        let p: &Integer = &self.curve.as_ref().unwrap().p;
        let mut den: Integer = Integer::ZERO + p - _q.y.as_ref().unwrap();
        if self.x == _q.x && self.y.as_ref().unwrap() == &den { 
            return o;
        }
        if self == _q && self.y.as_ref().unwrap() == &Integer::ZERO {
            return o;
        }

        let alpha: Integer;
        let a: &Integer = &self.curve.as_ref().unwrap().a;
        if self == _q {
            den = (Integer::from(2) * self.y.as_ref().unwrap()) % p;
            if den < Integer::ZERO {
                den += p;
            }
            if den != 1 || den != -1 {
                let mut t = euclides_gmp(&den, p);
                if t.1 < 0 {
                    t.1 += p;
                }
                //println!("t: {}", t.1);
                //println!("top: {}", 3 * self.x.clone().unwrap().square() + a);
                alpha = ((3 * self.x.clone().unwrap().square() + a) * t.1) % p;    
            } else {
                alpha = ((3 * self.x.clone().unwrap().square() + a) * den) % p;
            }
        } else {
            den = (_q.x.clone().unwrap() - self.x.clone().unwrap()) % p;
            if den < 0 {
                den += p;
            }
            if den != 1 || den != -1 {
                let mut t = euclides_gmp(&den, p);
                if t.1 < 0 {
                    t.1 += p;
                }
                alpha = ((_q.y.clone().unwrap() - self.y.clone().unwrap()) * t.1) % p;    
            } else {
                alpha = ((_q.y.clone().unwrap() - self.y.clone().unwrap()) / den) % p;
            }
        }
        
        //println!("alpha {}", alpha);
        ret.x = Some((alpha.clone().square() - self.x.clone().unwrap() - _q.x.clone().unwrap()) % p);
        if ret.x.as_ref().unwrap() < &Integer::ZERO {
            ret.x = Some(ret.x.unwrap() + p);
        }
        ret.y = Some((alpha * (self.x.unwrap() - ret.x.clone().unwrap()) - self.y.unwrap()) % p);
        if ret.y.as_ref().unwrap() < &Integer::ZERO {
            ret.y = Some(ret.y.unwrap() + p);
        }
        return ret;
    }
}

impl std::ops::Add<&Point> for Point {
    type Output = Self;

    fn add(self, rhs: &Point) -> Self::Output {
        let tst: Point = rhs.clone();
        return tst + self.clone();
    }
}

impl std::ops::Mul<Integer> for Point {
    type Output = Point;

    fn mul(self, rhs: Integer) -> Self::Output {
        let o: Point = Point {
            x: None,
            y: None,
            curve: self.curve.clone(),
        };

        //let mut i: Integer = Integer::ZERO;
        let mut r: Point = o;
        let mut q: Point = self.clone();
        let mut n: Integer = rhs.clone();
        while n > Integer::ZERO {
            if n.is_odd() {
                r = r + q.clone();
            }
            q = q.clone() + q;
            n = n / 2;
        }
        return r;
    }
}

pub fn floyd(x0: Point) -> (Integer, Integer) {

    let mut tortoise = x0.clone() * Integer::from(2);
    let mut hare = x0.clone() * Integer::from(3);
    while tortoise != hare {
        tortoise = tortoise + x0.clone();
        hare = hare + (x0.clone() * Integer::ZERO);
    }
    println!("found x_i = x_2i");
        
    let mut mu = 0;
    tortoise = x0.clone();
    while &tortoise != &hare {
        tortoise = tortoise + x0.clone();
        hare = hare + x0.clone();
        mu += 1;
        println!("{} ?= {}", &tortoise, &hare);
        println!("mu = {}", mu);
    }
    println!("Found mu");

    let mut lam = 1;
    hare = tortoise.clone() + x0.clone();
    while tortoise != hare {
        hare = hare + x0.clone();
        lam += 1;
    }
 
    return (Integer::from(lam), Integer::from(mu))
}

pub fn degree(x0: &Point) -> Integer {
    let mut r: Integer = Integer::from(1);
    let o: Point = Point{x: None, y: None, curve: x0.curve.clone()};
    let mut a: Point = x0.clone();
    while a != o {
        a = a + x0;
        r = r + Integer::from(1);
    }
    return r;
}
