use std::thread;
use std::sync::mpsc;  // multiple producer, single consumer
use std::time::Duration;

fn main() {

    {
        let (tx, rx) = mpsc::channel();
    
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
    
        let received = rx.recv().unwrap();         // block the main thread's execution and wait
        // let received = rx.try_recv().unwrap();  // nonblock, return a Result<T, E> immediately
        println!("Got: {received}");
    }

    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals: Vec<String> = vec!["hi", "from", "the", "thread"]
                .iter()
                .map(|s| String::from(*s))
                .collect();

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }

    {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals: Vec<String> = vec!["hi", "from", "the", "thread"]
                .iter()
                .map(|s| String::from(*s))
                .collect();

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals: Vec<String> = vec!["more", "messages", "for", "you"]
                .iter()
                .map(|s| String::from(*s))
                .collect();

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }

}
