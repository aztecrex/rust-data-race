


fn work_with_vector(ints: Vec<i32>) {
    println!("I own this now: {:?}", ints);
}

fn work_with_vector_and_give_it_back(ints: Vec<i32>) -> Vec<i32> {
    println!("I own this now but will give it back: {:?}", ints);
    ints
}


pub fn demo() {

    let v0 = vec![0,0,0];
    let v0_new_owner = v0;
    // println!("{:?}", v0);  <----- compile-time failure
    println!("I own this now: {:?}", v0_new_owner);

    let v1 = vec![1,2,3];
    work_with_vector(v1);
    // println!("{:?}", v1);  <----- compile-time failure

    let v2 = vec![4,5,6];
    let v2_returned = work_with_vector_and_give_it_back(v2);
    // println!("{:?}", v2);  <----- compile-time failure
    println!("And now I own it again: {:?}", v2_returned);

}
