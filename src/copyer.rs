
#[derive(Debug)]
struct Acceleration(f64);

#[derive(Debug,Clone,Copy)]
struct Velocity(f64);

pub fn demo() {

    let a1 = Acceleration(15.0);
    let a2 = a1;
    println!("I own it now: {:?}", a2);
    // println!("do not own anymore: {:?}", a1);

    let x = 12;
    let y = x;

    println!("I own this: {:?}", y);
    println!("I but seem to own it, too: {:?}", x);

    let v1 = Velocity(12.9);
    let v2 = v1;

    println!("I own this: {:?}", v2);
    println!("But I own the original: {:?}", v1);

}