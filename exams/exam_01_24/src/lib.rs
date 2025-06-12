// Sia data la struttura

/// ```rust
///LinkedList<T> definita come:
///pub struct LinkedList<'a, T> {
///	pub val: Option<T>,
///	pub next: Option<&'a Box<LinkedList<'a, T>>>,
///}
pub mod ex_1 { // Si definisca l’occupazione di memora di un elemento della lista e si indichi come sia possibile
    // definire il fine lista.
    //
    // COnsidero T = i32 e arc = 64bit
    // LinkedList rappresenta un nodo della lista dove avrà un nello stack due campi:
    // - val che è un option T quindi occupa il valore di tag 1 byte + occupatione memoria di T + eventuale padding nello stack
    // - next è un option di unriferimento a  Box di linked list, quindi nello stack occuperà esattamente quanto occopa quanto il rifermento a Box,
    //   dato che Option può sfruttare la logica null pointer optimization, perciò non ha la dimensione del tag. Quindi il referimento a Box è riferito a un
    //   Box istanziato nello stack che punta a un LinedList<T> nell'heap.

    // val= 1 + 4 = 5 + padding = 8
    // next= 0 + 8 = 8 +0padding = 8
    // tot = 16 byte

    // Per definire il fine lista basta semplicemente impostare prendere un linkedList che ha next = None
}

//Si definisca un esempio in cui, data la necessità di creare N thread, si possano evitare race-
//conditions nel momento in cui i thread debbano accedere in scrittura alla stessa risorsa. Si
//distingua il caso in cui tale risorsa sia uno scalare e quella in cui sia una struttura più articolata.
pub mod ex_2 {
    use std::sync::{Arc, Mutex,atomic::{AtomicUsize, Ordering }   };
    use std::thread;

    pub fn run_ex_2() {
        let shared_data_1 = Arc::new(Mutex::new(Vec::new()));
        let shared_data_2 = Arc::new(AtomicUsize::new(8));
        let mut handels = Vec::new();

        for i in 0..10 {
            let shared_data_1_clone = Arc::clone(&shared_data_1);
            let shared_data_2_clone = Arc::clone(&shared_data_2);

            let handle = thread::spawn(move || {
                let mut data = shared_data_1_clone.lock().unwrap();
                data.push(i);
                let mut data2 = shared_data_2_clone.load(Ordering::SeqCst);
                data2 += i;
                shared_data_2_clone.store(data2, Ordering::SeqCst);
				println!("Thread {}: shared_data_1 = {:?}, shared_data_2 = {}", i, *data, data2);

            });
            handels.push(handle);
        }

        for handle in handels {
            handle.join().unwrap();
        }
    }
}


// La struttura MultiChannel implementa il concetto di canale con molti mittenti e molti ricevitori.
// I messaggi inviati a questo tipo di canale sono composti da singoli byte che vengono recapitati
// a tutti i ricevitori attualmente collegati.

pub mod ex_4 {
	use std::result::Result;
	use std::sync::mpsc::{self, Receiver, SendError, Sender};
	use std::sync::{Arc, Mutex};

	pub struct MultiChannel {
		senders: Arc<Mutex<Vec<Sender<u8>>>>,
	}

	impl MultiChannel {
		pub fn new() -> Self {
			Self { senders: Arc::new(Mutex::new(Vec::new()))}
		}

		// crea un nuovo canale senza alcun ricevitore collegato
		pub fn subscribe(&self) -> Receiver<u8> {
			let (tx, rx) = mpsc::channel();
			let mut channel = self.senders.lock().unwrap();
			channel.push(tx);

			rx
		
		}
		// collega un nuovo ricevitore al canale: da quando
		// questo metodo viene invocato, gli eventuali byte
		// inviati al canale saranno recapitati al ricevitore.
		// Se il ricevitore viene eliminato, il canale
		// continuerà a funzionare inviando i propri dati
		// ai ricevitori restanti (se presenti), altrimenti
		// ritornerà un errore
		pub fn send(&self, data: u8) -> Result<(), SendError<u8>> {
			let guard = self.senders.lock().unwrap();

			if guard.len() > 0 {
				for sender in &*guard {
					sender.send(data)?;
				} 
				Ok(())
			} else {
				Err(SendError(data))
			}
		}
		// invia a tutti i sottoscrittori un byte
		// se non c'è alcun sottoscrittore, notifica l'errore
		// indicando il byte che non è stato trasmesso
	}
}

#[cfg(test)]
mod tests {
	use crate::ex_4::MultiChannel;
	use std::sync::mpsc::Receiver;
	use std::thread;

	#[test]
	fn test_multi_channel() {
		let channel = MultiChannel::new();
		let receiver1: Receiver<u8> = channel.subscribe();
		let receiver2: Receiver<u8> = channel.subscribe();

		let handle = thread::spawn(move || {
			channel.send(42).unwrap();
		});

		handle.join().unwrap();

		assert_eq!(receiver1.recv().unwrap(), 42);
		assert_eq!(receiver2.recv().unwrap(), 42);
	} 
}