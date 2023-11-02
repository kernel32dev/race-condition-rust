use std::time::Instant;


mod errado;
mod mutex;
mod atomic;

fn main() {
    let errado_comeco = Instant::now();
    errado::vai();
    println!("errado durou: {:?}", errado_comeco.elapsed());

    let mutex_comeco = Instant::now();
    mutex::vai();
    println!("mutex durou: {:?}", mutex_comeco.elapsed());

    let atomic_comeco = Instant::now();
    atomic::vai();
    println!("atomic durou: {:?}", atomic_comeco.elapsed());
}
