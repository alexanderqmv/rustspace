use std::io;



fn main()
{
    let mut vec: Vec<i32> = Vec::new();

    println!("Введите элементы массива (каждый с новой строки). Что бы закончить введите `done`");
    loop
    {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "done" { break; }

        let num: i32 = match input.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Неверный ввод. Попробуйте еще раз.");
                continue;
            }
        };
        vec.push(num); }

    vec.sort();
    println!("Отсортированный массив: {:?}", vec);
}
