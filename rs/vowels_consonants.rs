fn main() {
    let word = "hello,world!".to_string();
    let res = vowels(&word);
    println!("vowels : {} consonants: {}", res.0, res.1);
     
}

fn vowels(str: &String) -> (u32, u32) {
    // массив гласных букв
    let vowel = ['a','e', 'i', 'o','u'];
    // счетчик гласных букв
    let mut vowels_count = 0;

    // счетчик согласных букв
    let mut consonants_count = 0; 

    // итерируемся посимвольно в `str`
    for c in str.chars() {
        // если `c` из `str` символ содержит какой-то из гласных букв, 
        // то увеличиваем счетчик гласных букв на один
        if vowel.contains(&c.to_ascii_lowercase()) {
            vowels_count += 1;
        
        }else  {
            // если же не `c` не содержит какую-то букву из `vowels[]` 
            // то увеличиваем счетчик согласных букв
            consonants_count += 1;
        }
    }
    (vowels_count, consonants_count)
}
