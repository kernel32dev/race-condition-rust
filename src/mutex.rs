use std::sync::Mutex;

static CONTADOR: Mutex<u32> = Mutex::new(0);

pub fn vai() {
    let thread0 = std::thread::spawn(thread);
    let thread1 = std::thread::spawn(thread);
    let thread2 = std::thread::spawn(thread);
    let thread3 = std::thread::spawn(thread);
    thread0.join().unwrap();
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();

    let contador = *CONTADOR.lock().unwrap();
    println!("contador do mutex: {contador}");
}

fn thread() {
    for _ in 0..1000 {
        let mut lock = CONTADOR.lock().unwrap(); // pega o lock

        let mut valor_do_contador = *lock;

        std::thread::yield_now(); // simula um delay

        valor_do_contador += 1;

        std::thread::yield_now(); // simula um delay

        *lock = valor_do_contador;

        drop(lock); // solta o lock
    }
}
