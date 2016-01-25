use std::fs::File;
use std::io::Error;
use std::io::prelude::*;


#[derive(PartialEq, PartialOrd,Debug,Clone,Ord,Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn move_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn move_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn move_up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn move_down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
}

fn main() {
    println!("Hello, world! {}",create_point(&read_file_string().unwrap()).unwrap());
}


fn read_file_string() -> Result<String, Error> {
    let mut s = String::new();
    let mut f = try!(File::open("foo.txt"));
    try!(f.read_to_string(&mut s));
    Ok((s))
}


fn create_point(str: &str) -> Result<usize, Error> {
    println!("{:?}", str);
    let mut points: Vec<Point> = Vec::new();
    let mut moving_point: Point = Point::new(0, 0);
    for charac in str.trim().chars() {
        moving_point = match charac {
            // > 62
            '>' => moving_point.move_right(),

            // < 60
            '<' => moving_point.move_left(),
            // v 118
            'v' => moving_point.move_down(),
            // ^ 94
            '^' => moving_point.move_up(),

            x => {println!("{:?}",x );
                panic!("error")},
        };
        points.push(moving_point.clone());

    }

    points.sort();
    points.dedup();
    Ok((points.len()))
}

#[test]
fn check_houses() {
    assert!(create_point("^>v<").unwrap() == 4);
    assert!(create_point("^v^v^v^v^v").unwrap() == 2);
    assert!(create_point(">").unwrap() == 1);
}

#[test]
fn check_points_are_equal() {
    assert!(Point::new(2, 3) == (Point::new(2, 3)));
}

#[test]
fn check_points_are_not_equal() {
    println!("{:?}", Point::new(3, 2));
    assert!(Point::new(3, 2) != (Point::new(2, 3)));
}


#[test]
fn check_points_after_right() {
    println!("{:?}", Point::new(3, 2));
    assert!(Point::new(3, 2).move_right() == (Point::new(4, 2)));
}

#[test]
fn check_points_after_left() {
    println!("{:?}", Point::new(3, 2));
    assert!(Point::new(3, 2).move_left() == (Point::new(2, 2)));
}

#[test]
fn check_points_after_up() {
    println!("{:?}", Point::new(3, 2));
    assert!(Point::new(3, 2).move_up() == (Point::new(3, 3)));
}

#[test]
fn check_points_after_down() {
    println!("{:?}", Point::new(3, 2));
    assert!(Point::new(3, 2).move_down() == (Point::new(3, 1)));
}
