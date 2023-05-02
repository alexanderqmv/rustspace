#![allow(dead_code)]
use std::io;

pub const DEF_RPC_TIMEOUT_SECONDS: i32 = 30;

fn main()
{
    let reference = &4;
    match reference {
        &val => println!("Получаем значение через деструктуризацию: {:?}", val),
    }


    match *reference {
        val => println!("Получаем значение через разыменование: {:?}", val), }

    let _not_a_ref = 3;
    let ref _is_a_ref = 3;

    let value = 5;
    let mut mut_value = 7;
    match value { ref r => println!("Получили ссылку на значение: {:?}", r), }


    match mut_value
    {
        ref mut m => {
            *m += 10;
            println!("Мы добавили 10. `mut_value`: {:?}", m);
        },
    }



}
