use rand::Rng;

fn main() {
    let result = rand::thread_rng().gen_range(1..=20);

    println!("{}", result);
}
