use std::env::join_paths;
use std::ops::AddAssign;
use std::sync::Mutex;
use std::thread::{scope};

pub fn test_mutex() {
    let score = Mutex::new(0u16);

    // score += 5;                  // cannot be done
    // let unlocked_data = score.lock();
    // let mut data = unlocked_data.unwrap();
    // data.add_assign(5);
    //
    // println!("Data: {data}")
    // drop(data);

    let myfunc = || {
        println!("Thread 1 is waiting for mutex lock");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 1 is adding {i}");
        }
    };

    let myfunc2 = || {
        println!("Thread 2 is waiting for mutex lock");
        let mut data = score.lock().unwrap();
        // drop(data);
        // panic!("Error in the thread");
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 2 is adding {i}");
        }
    };

    // _ = spawn(myfunc).join();

    _ = scope(|s| {
        let handle1 = s.spawn(myfunc).join();
        let handle2 = s.spawn(myfunc2).join();

        if handle2.is_err() {
            println!("Thread2 has an error");
        }
        if handle1.is_err() {
            println!("Thread1 has na error");
        }
    });

    println!("{:?}", score.lock().unwrap());            // borrow of moved value: `score` [E0382]
}