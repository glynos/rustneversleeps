fn increment(x: &mut i32) -> i32 {
    *x += 1;
    *x
}

fn main() {
    let mut x = Box::new(42);
    println!("{x}"); // 42
    let mut y = increment(&mut x);
    let z = increment(&mut y);
    println!("{x} {y} {z}"); // 43 44 44
    println!("{:#b} {:p}", *x, x);
    println!("{:#b} {:p}", y, &y);
    println!("{:#b} {:p}", z, &z);
}
