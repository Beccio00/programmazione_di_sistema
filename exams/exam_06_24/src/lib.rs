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

// Si descriva il comportamento di questo programma.
// Se presenta delle problematiche, come può essere modificato?

/// ```
/// use std::sync::{Arc, Condvar, Mutex};
/// use std::thread;
/// use std::time::Duration;
/// fn main() {
///     let pair = Arc::new((Mutex::new(false), Condvar::new()));
///     let pair2 = Arc::clone(&pair);
///     thread::spawn(move || {
///         let (lock, cvar) = &*pair2;
///         let mut started = lock.lock().unwrap();
///         *started = true;
///         cvar.notify_one();
///     });
///     let (lock, cvar) = &*pair;
///     println!("Waiting ...");
///     thread::sleep(Duration::from_secs(1));
///     let mut started = lock.lock().unwrap();
///     started = cvar.wait(started).unwrap();
///     println!("End!");
/// }
pub mod ex_2 {
    // Nel codice vengono gestiti due thread, che accedono a un variabile condivisa booleana e comunicanano tra di loro attraverso una condvar.
    // Il primo è il thread principale che come tale non ho ha bisongo di essere spawnato in quanto lo fa già il main. Il seconod thread viene spawnato
    // e prende in possesso di clone di Arc che punta a una tupla formata dal Mutex della variabile booleana e Condavar. Il suo compito è quello di
    // modificare la variabile start e impostarla a true e poi svegliare il thread principale con notify_one(), così da simulare il fatto che il thread
    // ha iniziato la sua esecuzione e avvisare ciò attraverso il mutex. Il thread principaale invece prima esegue sleep per un secondo, simulando
    // l'escuzione successivamente esgue wait su il mutexguard che condivisono i due thread. Quindi il thread svegliara il thread principale nel momento
    // in cui finisce. In questo caso ci possono essere delle notifiche supurie e quindi sterted non è stato modificato e quindi è comunque false, per come
    // come è stato implementato il codice non c'è nessuno controllo quindi il thread prinicpale pensa che il secondo thread abbia già fatto quello che d
    // doveva fare. Per miglioare il codice basta sostuire wait(started) con wait_while(!*started) in modo che anche se viene svegliato ma started è ancora
    // false allora rimane in attesa.

    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;
    pub fn run_ex_2() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = Arc::clone(&pair);
        thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
        });
        let (lock, cvar) = &*pair;
        println!("Waiting ...");
        thread::sleep(Duration::from_secs(1));
        let mut started = lock.lock().unwrap();
        started = cvar.wait_while(started, |s| !*s).unwrap();
        println!("End!");
    }
}

//Il codice seguente genera un errore di compilazione: spiegare perché e indicare come
//modificare la struct S (attraverso l'aggiunta di tratti) per renderlo compilabile ed eseguibile.
///```
///#[derive(Debug)]
///struct S {
///    i: i32,
///}
///impl From<i32> for S {
///    fn from(value: i32) -> Self {
///        S { i: value }
///    }
///}
///fn main() {
///    let mut v = Vec::<S>::new();
///    let s = 42.into();
///    for i in 0..3 {
///        v.push(s);
///    }
///    println!("{:?}", v);
///}

pub mod ex_3 {
    // Il problema di questo codice è che in s inzialmente è in possesso di una variabile tipo S che non implementa il tratto Copy. Nel momento in cui
    // viene fatto v.push(s) la prima volta avviene con successo e la variabile s viene mossa all'interno dell'heap dove punta v, consumando s.
    // al secondo passaggio del ciclo in s non esiste più e v.push e il borrow cheker darà errore di compilazione. Per renderlo compatibile con la
    // la logica implementata basterebbe far si che struct S derivi anche i tratti Clone e Copy. In questo modo quando viene eseguito v.push
    // non viene mossa la variabile s ma viene inserita in v una copia di S.

    #[derive(Debug, Clone, Copy)]
    struct S {
        i: i32,
    }
    impl From<i32> for S {
        fn from(value: i32) -> Self {
            S { i: value }
        }
    }
    pub fn run_ex_3() {
        let mut v = Vec::<S>::new();
        let s = 42.into();
        for i in 0..3 {
            v.push(s);
        }
        println!("{:?}", v);
    }
}

//      Si realizzi l’implementazione della struttura dati Exchanger<T: Send> (e dei metodi e delle
//     funzioni necessarie) utile per realizzare una comunicazione bidirezionale.
///     Ciascun lato della comunicazione dispone di un’istanza della struttura [`Exchanger<T: Send>.`]
//     La comunicazione avviene invocando il metodo
///```
///     fn exchange(&self, t:T) -> Option<T>
//     che, una volta invocato, si blocca fino a quando non viene invocato il metodo corrispettivo
//     sulla struttura corrispondente al lato opposto della comunicazione, dopodiché restituisce il
//     valore che è stato passato come argomento al metodo corrispondente al lato opposto (che
//     farà altrettanto), sotto forma di Some(t).
//     Lo scambio può essere ripetuto un numero arbitrario di volte.
//     Se una delle due strutture formanti la coppia viene distrutta, un'eventuale chiamata, bloccata
//     sul metodo della struttura restante, terminerà restituendo il valore None.
//     Si implementi tale struttura in linguaggio Rust avendo cura che la sua implementazione
//     sia thread-safe

pub mod ex_4 {
    use std::sync::{Arc, Condvar, Mutex};

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum ExchangeState {
        Empty,
        Waiting,
    }

    pub struct SharedState<T: Send> {
        value1: Option<T>,
        value2: Option<T>,
        state: ExchangeState,
        dropped: bool,
    }

    #[derive(Clone)]
    pub struct Exchanger<T: Send> {
        shared: Arc<Mutex<SharedState<T>>>,
        condvar: Arc<Condvar>,
    }

    impl<T: Send> Exchanger<T> {
        pub fn new() -> (Self, Self) {
            let shared = Arc::new(Mutex::new(SharedState {
                value1: None,
                value2: None,
                state: ExchangeState::Empty,
                dropped: false,
            }));
            let condvar = Arc::new(Condvar::new());

            let ex1 = Exchanger {
                shared: Arc::clone(&shared),
                condvar: Arc::clone(&condvar),
            };

            let ex2 = Exchanger {
                shared: Arc::clone(&shared),
                condvar: Arc::clone(&condvar),
            };

            (ex1, ex2)
        }

        pub fn exchange(&self, value: T) -> Option<T> {
            let mut guard = self.shared.lock().unwrap();

            if guard.dropped {
                return None;
            }

            match guard.state {
                ExchangeState::Empty => {
                    guard.value1 = Some(value);
                    guard.state = ExchangeState::Waiting;

                    guard = self.condvar.wait_while(guard, |state| {
                        state.state == ExchangeState::Waiting && !state.dropped
                    }).unwrap();

                    if guard.dropped {
                        guard.value1 = None;
                        guard.value2 = None;
                        guard.state = ExchangeState::Empty;
                        None
                    } else {
                        let result = guard.value2.take();
                        guard.value1 = None;
                        guard.state = ExchangeState::Empty;
                        result
                    }
                }

                ExchangeState::Waiting => {
                    let other_value = guard.value1.take().unwrap();
                    guard.value2 = Some(value);
                    guard.state = ExchangeState::Empty;

                    self.condvar.notify_one();
                    Some(other_value)
                }
            }
        }
    }

    impl<T: Send> Drop for Exchanger<T> {
        fn drop(&mut self) {
            if let Ok(mut guard) = self.shared.lock() {
                guard.dropped = true;
                self.condvar.notify_all();
            }
        }
    }
}

