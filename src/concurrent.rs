use std::thread;
use std::time::Duration;
// use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn demo_naiive() {
    // let mut data = vec![1, 2, 3];
    //
    // for i in 0..3 {
    //     thread::spawn(move || {
    //         data[0] += i;
    //     });
    // }
    //
    // thread::sleep(Duration::from_millis(50));
    //
}

pub fn demo_will_not_work() {

    // let mut data = Rc::new(vec![1, 2, 3]);
    //
    // for i in 0..3 {
    //     // Create a new owned reference:
    //     let data_ref = data.clone();
    //
    //     // Use it in a thread:
    //     thread::spawn(move || {
    //         data_ref[0] += i;
    //     });
    // }
    //
    // thread::sleep(Duration::from_millis(50));
}


pub fn demo_still_will_not_work() {

    // let mut data = Arc::new(vec![1, 2, 3]);
    //
    //  for i in 0..3 {
    //      let data = data.clone();
    //      thread::spawn(move || {
    //          data[0] += i;
    //      });
    //  }
    //
    //  thread::sleep(Duration::from_millis(50));


}

pub fn demo() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
    println!("modified, shared data: {:?}", data);
}
