use std::thread;
use std::time::Duration;

use std:: sync::mpsc; // mpsc ==> multiple producer, single consumer
                          // a channel can have multiple sending ends that produce values 
                          // but ONLY one receiving end that consumes those values

// use std::sync::Mutex;
use std::rc::Rc;
use std::sync::{Mutex, Arc};
fn main() {
    // ---------------------- Part I ------------------------------
    // let handle = thread::spawn // return type of thread::spawn --> JoinHandle
    // (|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // // handle.join().unwrap(); // will finish the loop of spawn before executing loop of main thread
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap(); // allow the spawn thread continue to execute even after the main thread loop ending
    // or in another word: make the main thread wait for the spawn thread to end
    

    // ------------------------------ Part II ----------------------------
    // let v = vec![1,2,3];
    // let handle = thread::spawn( move // force closure to take the ownership of the values 
    //                                  // (for here, the "v") rather than allowing Rust to infer that
    //                                  // it should BORROW the values.
    //     || {
    //     println!("Here's a vector: {:?}", v); // Rust don't know how long the 
    //                                           // spawned thread will run, so it doesn't know
    //                                           // whether the reference to v will always be valid.
        
    // });

    // drop(v); // v is not valid example 
    //          // but cannot be dropped after move the ownership to another thread
    // handle.join().unwrap();


    // ------------------------------ Part III --------------------------
    

    
    // let (tx, rx) = mpsc::channel(); // returns a tuple

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     // println!("Val is {}", val); // access val in spawn thread after sending it down
    //                                 // this will fail (compile-time error) as the ownership 
    //                                 // has been transferred
    //                                 // send() take the ownership ---> then receiver takes ownership
    // });

    // let received = rx.recv().unwrap(); 
    // // let received = rx.try_recv().unwrap(); 
    // println!("Got: {}", received);
    
    // ----------------------------- End of Part III -------------------

    // ------------------------------Part IV ---------------------------
    // let (tx, rx) = mpsc::channel();
    // let tx1 = mpsc::Sender::clone(&tx);
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });


    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });


    // for received in rx {
    //     println!("Got: {}", received);
    // }
    // ---------------------------End of Part IV-------------------------

    // ------------------------------Part V------------------------------
    // mutex allows only one thread to access some data at any given time.
    // to do so, a thread must:
    //      1. asking to acquire the mutex's lock
    //              lock: (a data structure that is part of the mutex that keeps track of who
    //                     currently has exclusive access to the data)
    // mutex is hard to use: 
    //      a. must attempt to acquire the lock before using data
    //      b. when you done, need to lock the data so other threads can acquire the lock


    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m);

    //let counter = Mutex::new(0);
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());
    // error in above


    

    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();

    //     *num += 1;
    // });
    // handles.push(handle);

    // let handle2 = thread::spawn(move || {
    //     let mut num2 = counter.lock().unwrap();
    //     *num2 += 1;
    // });
    // handles.push(handle2); // make sure the thread finishes

    
    // let counter = Rc::new(Mutex::new(0)); // crate a reference counted value
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
    // above sent and move are not SAFE !!!
    

    // let counter = Arc::new(Mutex::new(0)); // Arc: atomically reference counted type
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    // // counter is immutable but we could get a mutable reference to the value inside it.
    // ----------------------------End of Part V------------------------------

    // -------------------------------Part VI---------------------------------
    //Rc<T> is implemented in the case of single thread situation
    // as it cannot be sent: if cloned an Rc<T> value and try to transfer the onwership, both
    // thread update the reference count at the same time

    // any type "T" is "Sync" if &T is "Send"


    // many good (even better than standard )implementation of concurrency are implemented in crates
    // so search for them when necessary for a even better performance and security!
}


