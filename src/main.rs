#![warn(clippy::all, clippy::pedantic, clippy::style, rust_2018_idioms)]

mod finite_fields;
mod elliptic_curve;

use elliptic_curve::PointType;

use crate::{finite_fields::macros::felt, elliptic_curve::Point};

fn fmt_point(pt: &Point) -> String {
    let x_repr = match pt.x.clone() {
        PointType::Infinity => String::from("INF"),
        PointType::Normal(x) => format!("{}", x.inner()),
    };

    let y_repr = match pt.y.clone() {
        PointType::Infinity => String::from("INF"),
        PointType::Normal(y) => format!("{}", y.inner()),
    };

    format!("({}, {})", x_repr, y_repr)
}

fn main() {
    let prime: u64 = 223;
    let a = felt!(0, prime);
    let b = felt!(7, prime);
    let x = felt!(47, prime);
    let y = felt!(71, prime);
    let pt = Point::try_from_felts(x, y, a, b).expect("Point is not on the curve");

    for s in 1..21u32 {
        let pt2 = pt.clone() * s;
        println!("{} * {} = {}", s, fmt_point(&pt), fmt_point(&pt2));
    }

    println!("Hello, world!");
}
