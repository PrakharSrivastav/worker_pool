use worker_pool::Pool;

fn main() {
    let pool = Pool::new(3);

    for i in 0..=30 {
        pool.execute(move || process(i))
    }
}

fn process(i: i32) {
    println!("processing item [{}] ", i)
}
