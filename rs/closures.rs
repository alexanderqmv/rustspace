/** 
    * Fn: замыкание захватывает по ссылке (&T)
    * FnMut: замыкание захватывает по изменяемой ссылке (&mut T)
    * FnOnce: замыкание захватывает по значению (T)
*/

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;
    let greeting = "hello";
    // `to_owned` преобразует заимствованные данные в собственные.
    let mut farewall = "bye".to_owned();
    // guard

    let diary = || {
        println!("I said {}.", greeting);

        farewall.push_str("!!!");
        println!("Than i screamed {}", farewall);
        println!("Now i can sleep. ZzzZzzzZ");
        mem::drop(farewall);
    };
    apply(diary);

    let double = |x| 2 * x;
    println!("Doubled 3: {}", apply_to(double));
}
