fn main() {
    let numbers: Vec<_> = (0..=10).filter(|i| i % 2 == 0).collect();

    for n in numbers {
        println!("{}", n);
    }
    let num = (0..=100).filter(|i| i % 2 == 0);

    for n in num {
        println!("{}", n);
    }

    let mut arr = [0; 11];
    for i in (0..=10).rev() {
        arr[i] = 10 - i;
    }
    println!("{:?}",arr);
}
