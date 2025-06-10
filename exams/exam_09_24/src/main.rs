use exam_09_24::ex_1::*;
use exam_09_24::ex_2::*;
use exam_09_24::ex_3::*;
fn main() {
    //run_ex_1();

    run_ex_2();

    let string1 = String::from("torino");
    let result;
    {
        let string2 = String::from("2024");
        result = fun1(string1.as_str(), string2.as_str());
    }
    println!("The fun string is {}", result);


    let string1 = String::from("ciao mamma");
    let result;
    
        let string2 = String::from("Torino");
        result = fun2(string1.as_str(), string2.as_str());
    
    println!("The fun string is {}", result);
}

