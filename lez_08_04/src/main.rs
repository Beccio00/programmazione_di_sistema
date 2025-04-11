#[derive(Debug)]
struct S(String);

impl Drop for S {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let mut v = vec![S("A".to_string()), S("B".to_string()), S("C".to_string())];

    for s in v {
        println!("Processing {}", s.0);
        if s.0 == "A" {
            println!("Found A breaking out from the loop");
            break;
        }
        //Se esco prima del loop vengono comunque droppate le che che avanzano
        
    }

    //println!("{:?}", v); non lo posso vedere perchè ormai non esiste più v 
}