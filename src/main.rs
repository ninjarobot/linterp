//use std::io;
extern crate num_traits;

use num_traits::identities::zero;

use std::fmt;

/// A 2D coordinate.
pub struct Coordinate<T> {
    /// x-position
    x:T,
    /// y-position
    y:T
}

impl <T: fmt::Display> fmt::Display for Coordinate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "x:{} y:{}", self.x, self.y)
    }
}

/// Interpolates the y-position given two 2D coordinates and an x-position.
fn linterp<T>(c1: &Coordinate<T>, c2: &Coordinate<T>, x:T) -> Coordinate<T> where T: num_traits::Num + Copy {
    let y = c1.y + (x - c1.x) * (c2.y - c1.y) / (c2.x - c1.x);
    Coordinate {
        x,
        y
    }
}

/// Averages a vector of 2D coordinates.
fn average(coordinates: &Vec<Coordinate<f64>>) -> Coordinate<f64> {
    let intermediate = coordinates.iter().fold(
        Coordinate::<f64> {x:zero(), y:zero()},
        |acc, coord| Coordinate {
            x: acc.x + coord.x,
            y: acc.y + coord.y
        }
    );
    let len = coordinates.len() as f64;
    Coordinate::<f64> {
        x: intermediate.x / len,
        y: intermediate.y / len
    }
}

fn main() {
    let c1 = Coordinate { x: 90.0, y: 100.0 };
    let c2 = Coordinate { x: 96.0, y: 110.0 };
    let c3 = Coordinate { x: 82.0, y: 118.0 };
    let c4 = Coordinate { x: 87.0, y: 103.0 };
    let coordinates = vec![c1, c2, c3, c4];
    let averaged = average(&coordinates);
    let c1 = Coordinate { x: 90, y: 100 };
    let c2 = Coordinate { x: 96, y: 109 };
    let interpolated = linterp(&c1, &c2, 92);
    //let mut enteredCoordinate = String::new();
    //std::io::stdin().read_line(&mut enteredCoordinate);

    println!("Averaged value: (x:{}, y:{})", averaged.x, averaged.y);
    println!("Interpolated value: ({})", interpolated);
}