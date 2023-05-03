fn give_comonner(gift: Option<&str>) {
    match gift {
        Some("bear") => println!("Omg!"),
        Some(inner) => println!("{} is so good!", inner),
        None => println!("No gift? I am sad!"),
    }
}
fn give_princess(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "bear" {
        panic!("Omg!");
    }
    println!("I love {}", inside);
}

fn main() {
    let food = Some("potato");

    let bear = Some("bear");
    let void = None;
    give_comonner(food);
    give_comonner(bear);
    give_comonner(void);
    let bird = Some("burger");

    let nothing = None;
    give_princess(bird);

    give_princess(nothing);
}
