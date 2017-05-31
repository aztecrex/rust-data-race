
fn work_with_borrowed_vector(ints: &Vec<i32>) {
    println!("I am just borrowing these in a function: {:?}", ints);
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


}