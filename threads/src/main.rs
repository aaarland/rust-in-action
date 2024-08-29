use std::{ env, thread, time};
fn main() {
    let args = env::args();
    if let Some(my_arg) = args.into_iter().nth(1){
        match my_arg.as_str(){
            "sleep" => sleep_threads(),
            "loop" => spin_loop(),
            _ => println!("Option {} is not available, use 'sleep' or 'loop'", my_arg)
        }
    }
}

fn sleep_threads() {
    let pause = time::Duration::from_millis(20);
    for n in 1..=1000 {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();
        for _m in 0..n {
            let handle = thread::spawn(move || {
                thread::sleep(pause);
            });
            handlers.push(handle);
        }
        while let Some(handle) = handlers.pop() {
            handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}

fn spin_loop() {
    let pause = time::Duration::from_millis(20);
    for n in 1..=1000 {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();
        for _m in 0..n {
            let handle = thread::spawn(move || {
                let start = time::Instant::now();
                while start.elapsed() < pause {
                    thread::yield_now();
                }
                thread::sleep(pause);
            });
            handlers.push(handle);
        }
        while let Some(handle) = handlers.pop() {
            handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }


}
