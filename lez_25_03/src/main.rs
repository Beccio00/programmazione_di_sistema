/* #2 it is the same thing
fn update(v: &mut Vec<&str>) {
    let s = "World".to_string();
    v.push(&s);
}
*/
/*#3 ther is a borrow... 
fn update<'a>(v: &mut Vec<&'a str>) {
    let s = "World".to_string();
    v.push(&'a s);
}
*/

use std::path::Prefix;

/*#4 */
fn update<'a>(v: &mut Vec<&'a str>, s: &'a str) {
    
    v.push(s);
}

fn create_operator(op: &str) -> impl Fn(i32, i32) -> i32 {
    match op {
        "+" => |x, y| {return x + y},
        "-" => |x, y| {if x>y {x-y} else {x - y}},
        "*" => |x, y| x * y,
        "/" => |x, y| x / y,
        _ => panic!("unknow operator"),
        
    }
}



struct Matricola {
    prefisso: i32,
    counter: i32,
}

impl Matricola {
    fn next (&mut self) -> String {
        self.counter += 1;
        format!("{}{:03}", self.prefisso, self.counter)
    }

    fn create_counter(&str) -> impl FnMut() _> String {
        let mut counter = 0;
        move || {
            counter += 1;
            format!("{}{:03}")
        }
    };
     /* Le funzioni lambda in rust possono catturare la memeoria nel conteesto in cui lavorano, la memeoria la da la CHIUSRA */
}


fn main() {
    let mut v = Vec::<&str>::new();
    v.push("Hello");
    
    /* #1
    {
        let s = "World".to_string();
        v.push(&s); //not live enough
    }
    */

    /*#5 non si può mettere un riferimento a qualcosa che svanisce subito anche se la funzione push compila senza problemi 
    update(&mut v, & "world".to_string());
    */

    /*#6 in questo caso i tempo di vita delle varibaili dura abbastanza */
    let s ="world".to_string();
    update(&mut v, & s);
    v.push("!");

    /*Sono tutte cose che in c sarebbero stata accetate dal compilatore ma il fatto è che le funzioni sarebbero state undefine */



    //ES_2: programmazione funzionale
    let f = |x: i32| x + 1;
    println!("1 -> {}", f(1));


    let mut f1 = create_counter("s");
    let mut f2 = create_counter("f");
    println!("{}", f1());
    println!("{}", f1());
    

    
}
