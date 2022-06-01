fn increment(x: &mut i32) -> i32 {
    *x += 1;
    *x
}

fn main() {
    let mut x: i32 = 42;			// Figure 1
    println!("{:?}", x); 		// 42
    println!("&x (main, before increment): {:p}", &x); // Figure 2
    let mut y = increment(&mut x);	// Figure 3
    println!("&x (main, after increment): {:p}", &x);

    println!("&y (main, before increment): {:p}", &y);
    let z = increment(&mut y);
    println!("&y (main, after increment): {:p}", &y);

    println!("{x} {y} {z}"); 		// 43 44 44
    println!("{:#b} {:p}", x, &x); // 0b101100 0x7ff7bda1d0dc
    println!("{:#b} {:p}", y, &y); // 0b101011 0x7ff7bda1d120
    println!("{:#b} {:p}", z, &z); // 0b101100 0x7ff7bda1d124
}
