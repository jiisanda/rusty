use std::ops::AddAssign;
use std::sync::Mutex;
use std::thread::{scope, sleep};
use std::time::Duration;

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
            sleep(Duration::from_millis(430));
        }
    };

    let myfunc2 = || {
        loop {
            println!("Thread 2 is waiting for mutex lock");
            let  guard = score.try_lock();

            if guard.is_ok() {
                let mut data = guard.unwrap();
                for i in 1..10 {
                    data.add_assign(i);
                    println!("Thread 2 is adding {i}");
                }
                break;
            }

            sleep(Duration::from_millis(300));
        }
        // drop(data);
        // panic!("Error in the thread");
    };

    // _ = spawn(myfunc).join();

    _ = scope(|s| {
        _ = s.spawn(myfunc);
        _ = s.spawn(myfunc2);

    });

    println!("{:?}", score.lock().unwrap());            // borrow of moved value: `score` [E0382]
}