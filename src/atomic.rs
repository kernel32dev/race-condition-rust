use std::sync::atomic::{AtomicU32, Ordering};

static CONTADOR: AtomicU32 = AtomicU32::new(0);

pub fn vai() {
    let thread0 = std::thread::spawn(thread);
    let thread1 = std::thread::spawn(thread);
    let thread2 = std::thread::spawn(thread);
    let thread3 = std::thread::spawn(thread);
    thread0.join().unwrap();
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();

    let contador = CONTADOR.load(Ordering::Relaxed);
    println!("contador do atomic: {contador}");
}

fn thread() {
    for _ in 0..1000 {
        CONTADOR.fetch_add(1, Ordering::SeqCst);
    }
}
