use rand::Rng;

fn main()
{
    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(1..101);
    println!("Random number: {}", random_number);

}
