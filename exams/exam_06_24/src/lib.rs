//si consideri il programma seguente che riporta la numerazione delle linee di codice.

/// ```
/// fn main() {
///     let numbers = vec![1, 2, 3, 4, 5, 8];  
///     let res = numbers
///         .iter()                         // Riga 5
///         .filter(|&x| x % 2 == 0)       // Riga 6
///         .zip('a'..'z');                // Riga 7
///     let last = res
///         .clone()                       // Riga 10
///         .map(|(a, b)| { format!("{b}{a}") })
///         .last();
///     println!("last: {:?}", last);
///     println!("res: {:?}", res.count());
/// }
/// ```
// Che cosa stampa questo codice?
// Che cosa fanno le istruzioni alle righe 5,6,7?
// Che cosa capita se si omette la riga 10? Perché?

pub mod ex_1 {
    // questo codice stamperebbe una cosa del tipo:
    // "last: Some(c8)"
    // "res: 3"
    // dato che appunto perchè in res viene salvato un interatore che conitiene gli elementi del vettore con modulo 2 (2,4,8) che vengono zippati con le
    // lettere dell'alfabeto quindi (a,b,c). Però non viene fatto la collect(), quindi dato che zip è solo un adattore, l'interatore non viene consumato
    // e res rimane un interatore e non viene trasformato in altri tipi.
    // In last viene salvato invece una copia dell'iteratore a cui viene cambiato il formato quindi mettendo prima 
    // la lettera poi il numero e poi viene preso l'ultimo elemento.
    // Se si ometesse la riga 10 last prenderebbe il possesso di res e lo consuma, di
    // conseguenza l'ultima riga andrebbe in constrasto con il borrow checker dato che res non esiste più. Invece con .clone() viene fatta una deep copy
    // di res e quindi rimangono in vita entrambe.

    pub fn run_ex_1() {
        let numbers = vec![1, 2, 3, 4, 5, 8];
        let res = numbers
            .iter() // Riga 5
            .filter(|&x| x % 2 == 0) // Riga 6
            .zip('a'..'z'); // Riga 7 
        let last = res
            .clone() // Riga 10
            .map(|(a, b)| format!("{b}{a}"))
            .last();
        println!("last: {:?}", last);
        println!("res: {:?}", res.count());
    }
}

pub mod ex_2 {}
