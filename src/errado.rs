
static mut CONTADOR: u32 = 0;

pub fn vai() {
    let thread0 = std::thread::spawn(thread);
    let thread1 = std::thread::spawn(thread);
    let thread2 = std::thread::spawn(thread);
    let thread3 = std::thread::spawn(thread);
    thread0.join().unwrap();
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();

    let contador = unsafe {CONTADOR};
    println!("contador do errado: {contador}");
}

fn thread() {
    for _ in 0..1000 {
        unsafe {
            let mut valor_do_contador = CONTADOR;

            std::thread::yield_now(); // simula um delay

            valor_do_contador += 1;

            std::thread::yield_now(); // simula um delay

            CONTADOR = valor_do_contador;
        }
    }
}
