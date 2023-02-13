use std::io;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main(){
    //loop{
        //****Convert English to Russian  keyboard layout****//
        let mut input_text = String::new();
        let set_of_letter:HashMap<char,char> =HashMap::from([//33 buttons
            ('q', 'й'), ('w', 'ц'), ('e', 'у'), ('r', 'к'), ('t', 'е'), ('y', 'н'), ('u', 'г'), ('i', 'ш'), ('o', 'щ'), ('p', 'з'), ('[', 'х'), (']', 'ъ'),
            ('a', 'ф'), ('s', 'ы'), ('d', 'в'), ('f', 'а'), ('g', 'п'), ('h', 'р'), ('j', 'о'), ('k', 'л'), ('l', 'д'), (';', 'ж'), ('\'', 'э'),
            ('z', 'я'), ('x', 'ч'), ('c', 'с'), ('v', 'м'), ('b', 'и'), ('n', 'т'), ('m', 'ь'), (',', 'б'), ('.', 'ю'), ('/', '.'),
       ]);

        println!("Hello! Write text which You want to convert:");
        match io::stdin().read_line(&mut input_text){
            Ok(_)=>{// input success
                    println!("\nText accepted for conveting\nPlease,Wait..."); 
            },
            Err(e)=>{eprintln!("Error of input. Please try again. {e}");//input Error
        }}
        let start = Instant::now(); // start of countdown
        let lower_text = input_text.to_lowercase();
        let char_text:Vec<char>=lower_text.chars().collect(); //cjnvert input text to char
        let mut length =char_text.len(); 
        if !char_text.is_empty(){
        //for (engl:&char, rusl:&char) in {
        for i in 0..(&length-1){   
            match set_of_letter.get(/*!!*/){
                Some(letter)=>{
                    //let mut temp = set_of_letter.contains_key(char_text[i]);
                    char_text.remove(i);
                    //char_text.insert(i,temp);
                },
                None=>{
                    println!("English alphabet does not contain this letter: {}", &i);
                }
            }
        }
        }else{
            eprintln!("Error.Char_text is empty. Try again");//Error
        }

        let duration =start.elapsed();//end of countdown
        println!("Amount of symbols in text: {}", length);
        println!("Time to execute the funtion: {0:?}", duration);
    }
