
fn work_with_borrowed_vector(ints: &Vec<i32>) {
    println!("I am just borrowing these in a function: {:?}", ints);
}

fn modify_borrowed_vector(ints: &mut Vec<i32>) {
    ints[0] += 100;
}

pub fn demo() {

    let v0 = vec![100, 100, 100];
    let v0_borrowed = &v0;
    let v0_another_borrower = &v0;
    println!("I borrowed these: {:?}", v0);
    println!("So did I: {:?}", v0_borrowed);
    println!("I still own them: {:?}", v0_another_borrower);

    let v1 = vec![7,8,9];
    work_with_borrowed_vector(&v1);
    println!("See? I still own them: {:?}", v1);

    let mut v2 = vec![10, 11, 12];
    println!("I can read this: {:?}", v2);
    println!("I can read it again: {:?}", v2);
    // let v2_borrowed =  &v2;
    modify_borrowed_vector(&mut v2);
    println!("I can still read it: {:?}", v2);
    {
        let v2_borrowed_for_write = &mut v2;
        v2_borrowed_for_write[1] = 9999;
        // println!("try to read: {:?}", v2);
        println!("I can read from the borrower: {:?}", v2_borrowed_for_write);
    }
    println!("Whew, now I can read it: {:?}", v2);


}