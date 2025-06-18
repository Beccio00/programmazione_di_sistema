// In un sistema concorrente, ciascun thread può pubblicare eventi per rendere noto ad altri thread quanto sta
// facendo. Per evitare un accoppiamento stretto tra mittenti e destinatari degli eventi, si utilizza un Dispatcher:
// questo è un oggetto thread-safe che offre il metodo dispatch(msg: Msg) mediante il quale un messaggio di
// tipo generico Msg (soggetto al vincolo di essere clonabile) viene reso disponibile a chiunque si sia
// sottoscritto.
// Un thread interessato a ricevere messaggi può invocare il metodo subscribe() del Dispatcher: otterrà come
// risultato un oggetto di tipo Subscription mediante il quale potrà leggere i messaggi che da ora in poi
// saranno pubblicati attraverso il Dispatcher.
// Per ogni sottoscrizione attiva, il Dispatcher mantiene internamente l'equivalente di una coda ordinata (FIFO)
// di messaggi non ancora letti. A fronte dell'invocazione del metodo dispatch(msg:Msg), il messaggio viene
// clonato ed inserito in ciascuna delle code esistenti. L'oggetto Subscription offre il metodo bloccante
///```
/// read() -> Option
// se nella coda corrispondente è presente almeno un messaggio, questo viene rimosso e restituito;
// se nella coda non è presente nessun messaggio e il Dispatcher esiste ancora, l'invocazione si blocca fino a
// che non viene inserito un nuovo messaggio; se invece il Dispatcher è stato distrutto, viene restituito None.
// La distruzione del Dispatcher non deve impedire la consumazione dei messaggi già recapitati ad una
// Subscription, ma non ancora letti; parimenti, la distruzione di una Subscription non deve impedire al
// Dispatcher di consegnare ulteriori messaggi alle eventuali altre Subscription presenti.
// Si implementino le strutture dati Dispatcher e Subscription nel linguaggio Rust

pub mod dispatcher {
    use std::{
        fmt::Debug,
        sync::{
            Mutex,
            mpsc::{self, Receiver, Sender, channel},
        },
    };

    #[derive(Clone, Debug)]
    pub struct Msg<T: Clone + Debug> {
        pub value: T,
    }

    pub struct Dispatcher<T: Clone + Debug> {
        subscriptions: Mutex<Vec<Sender<Msg<T>>>>,
    }

    impl<T: Clone + Debug> Dispatcher<T> {
        pub fn new() -> Self {
            Dispatcher {
                subscriptions: Mutex::new(Vec::new()),
            }
        }

        pub fn dispatch(&self, msg: Msg<T>) {
            let mut guard = self.subscriptions.lock().unwrap();
            guard.retain(|sender| sender.send(msg.clone()).is_ok());
        }

        pub fn subscribe(&self) -> Subscription<T> {
            let (tx, rx) = channel();

            let mut guard = self.subscriptions.lock().unwrap();

            guard.push(tx);
            Subscription::new(rx)
        }
    }

    pub struct Subscription<T: Clone + Debug> {
        receiver: Receiver<Msg<T>>,
    }

    impl<T: Clone + Debug> Subscription<T> {
        pub fn new(receiver: Receiver<Msg<T>>) -> Self {
            Subscription { receiver: receiver }
        }

        pub fn read(&self) -> Option<Msg<T>> {
            self.receiver.recv().ok()
        }
    }
}


#[cfg(test)]
mod test {
    use crate::dispatcher::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_broadcast_to_all_subscriptions() {
        let dispatcher = Arc::new(Dispatcher::<String>::new());
        let received_messages = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        
        for i in 0..3 {
            let dispatcher_clone = Arc::clone(&dispatcher);
            let messages_clone = Arc::clone(&received_messages);
            
            let handle = thread::spawn(move || {
                let subscription = dispatcher_clone.subscribe();
                
                if let Some(msg) = subscription.read() {
                    let mut guard = messages_clone.lock().unwrap();
                    guard.push(format!("Thread {}: {}", i, msg.value));
                }
            });
            handles.push(handle);
        }
        
        thread::sleep(Duration::from_millis(50));
        
        dispatcher.dispatch(Msg { value: "Test".to_string() });
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let messages = received_messages.lock().unwrap();
        assert_eq!(messages.len(), 3);
        
        for msg in messages.iter() {
            assert!(msg.contains("Test"));
        }
    }

    #[test]
    fn test_dispatcher_destruction() {
        let subscription = {
            let dispatcher = Arc::new(Dispatcher::<String>::new());
            let sub = dispatcher.subscribe();
            
            dispatcher.dispatch(Msg { value: "Before destruction".to_string() });
            sub
        };
        
        let msg = subscription.read();
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().value, "Before destruction");
        
        let msg2 = subscription.read();
        assert!(msg2.is_none());
    }
}

mod ranking_barrier {
    use std::sync::Condvar;

    pub struct RankingBarre {
        n_threads: usize,
        cv: Condvar,
    }
}