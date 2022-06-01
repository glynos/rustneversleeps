fn increment(x: &mut i32) -> i32 {
    *x += 1;
    *x
}

static mut X: i32 = 42;

fn main() {
    println!("{}", unsafe { X }); // 42
    let mut y = increment(unsafe { &mut X });
    let z = increment(&mut y);
    println!("{} {} {}", unsafe { X }, y, z); // 43 44 44
    println!("{:#b} {:p}", unsafe { X }, unsafe { &X });
    println!("{:#b} {:p}", y, &y);
    println!("{:#b} {:p}", z, &z);
}
