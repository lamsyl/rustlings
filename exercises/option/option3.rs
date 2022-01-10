// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // let y: Option<Point> = None;

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    println!("y = {:?}", y); // Fix without deleting this line.
}
