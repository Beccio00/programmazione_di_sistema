// La struct MpMcChannel<E: Send> è una implementazione di un canale su cui possono
// scrivere molti produttori e da cui possono attingere valori molti consumatori.

// Tale struttura offre i seguenti metodi:
    // new(n: usize) -> Self //crea una istanza del canale basato su un buffer circolare di "n" elementi
    
    // send(e: E) -> Option<()> //invia l'elemento "e" sul canale. Se il buffer circolare è pieno, attende 
    //senza consumare CPU che si crei almeno un posto libero in cui depositare il valore

    //  //Ritorna:
    //  // - Some(()) se è stato possibile inserire il valore nel buffer circolare
    //  // - None se il canale è stato chiuso (Attenzione: la chiusura può avvenire anche
          // mentre si è in attesa che si liberi spazio) o se si è verificato un errore interno
// recv() -> Option<E> //legge il prossimo elemento presente sul canale. Se il buffer
// circolare è vuoto,
//  //attende senza consumare CPU che venga depositato
// 7/2/23 5:32 PM Pagina 5 di 9
// almeno un valore
//  //Ritorna:
//  // - Some(e) se è stato possibile prelevare un valore dal
// buffer
//  // - None se il canale è stato chiuso (Attenzione: se,
// all'atto della chiusura sono
//  // già presenti valori nel buffer, questi devono essere
// ritornati, prima di indicare
//  // che il buffer è stato chiuso; se la chiusura avviene mentre si è in attesa di un valore,
//  // l'attesa si sblocca e viene ritornato None) o se si è
// verificato un errore interno.
// shutdown() -> Option<()> //chiude il canale, impedendo ulteriori invii di valori.
//  //Ritorna:
//  // - Some(()) per indicare la corretta chiusura
//  // - None in caso di errore interno all'implementazione del metodo.

// Si implementi tale struttura dati in linguaggio Rust, senza utilizzare i canali forniti dalla
// libreria standard né da altre librerie, avendo cura di garantirne la correttezza in
// presenza di più thread e di non generare la condizione di panico all'interno dei suoi
// metodi

mod mp_mc_channel {
    use std::{collections::VecDeque, sync::{Condvar, Mutex}};

    #[derive(PartialEq)]
    enum ChannelState {
        Close,
        Open,
    }

    pub struct MpMcChannel<E: Send> {
        buffer: Mutex<(ChannelState, VecDeque<E>)>,
        size: usize,    
        cv: Condvar,
    }

    impl<E: Send> MpMcChannel<E> {

        pub fn new(n: usize) -> Self{
            Self { 
                buffer: Mutex::new((ChannelState::Open, VecDeque::with_capacity(n))), 
                size: n, 
                cv: Condvar::new(),
            }
        }

        pub fn send(&self, e: E) -> Option<()> {
            let try_lock = self.buffer.lock();

            match try_lock {
                Err(_) => None,
                Ok(guard) => {
                    let data = self.cv.wait_while(guard, |(state, buf)|{ 
                        *state == ChannelState::Open && buf.len() == self.size
                    });

                    match data {
                        Err(_) => None,
                        Ok(mut guard) => {
                            match (*guard).0 {
                                
                                ChannelState::Open => {
                                    (*guard).1.push_back(e);
                                    drop(guard);
                                    self.cv.notify_all();
                                    Some(())
                                }

                                ChannelState::Close => {
                                    None
                                }
                            }

                        }
                    }
                } 
            }


            
        }

        pub fn recv(&self) -> Option<E> {
            let try_lock = self.buffer.lock();

            match try_lock {
                Err(_) => None,
                Ok(guard) => {
                    let data = self.cv.wait_while(guard, |(state, buf)|{
                        *state == ChannelState::Open && buf.len() == 0
                    });

                    match data {
                        Err(_) => None,
                        Ok(mut guard) => {
                            match (*guard).1.len() {
                                0 => None,
                                _ => {
                                    let result = (*guard).1.pop_front();
                                    drop(guard);
                                    self.cv.notify_all();
                                    result 
                                }, 

                            } 

                        }
                        
                    }
                }
            }
        }

    
        pub fn shutdown(&self) -> Option<()> {
            let try_lock = self.buffer.lock();
            match  try_lock{
                Err(_) => None,
                Ok(mut guard) => {
                    (*guard).0 = ChannelState::Close;
                    drop(guard);
                    self.cv.notify_all();
                    Some(())
                }
            }
        }
    }

}

#[cfg(test)]
mod test {

    use crate::mp_mc_channel::MpMcChannel;


    #[test]
    fn test_send_recv_single_thread() {
        let chan = MpMcChannel::new(2);
        assert_eq!(chan.send(42), Some(()));
        assert_eq!(chan.recv(), Some(42));

        chan.shutdown();
        assert_eq!(chan.recv(), None); 
    }

    #[test]
    fn test_shutdown() {
        let chan = MpMcChannel::new(1);
        assert_eq!(chan.send(1), Some(()));
        assert_eq!(chan.shutdown(), Some(()));
        assert_eq!(chan.send(2), None); 
        assert_eq!(chan.recv(), Some(1)); 
        assert_eq!(chan.recv(), None);    
    }

    #[test]
    fn test_multi_thread() {
        use std::sync::Arc;
        use std::thread;
    
        let chan = Arc::new(MpMcChannel::new(2));
        let chan2 = chan.clone();
    
        let producer = thread::spawn(move || {
            for i in 0..5 {
                assert_eq!(chan.send(i), Some(()));
            }
            chan.shutdown();
        });
    
        let consumer = thread::spawn(move || {
            let mut sum = 0;
            while let Some(val) = chan2.recv() {
                sum += val;
            }
            sum
        });
    
        producer.join().unwrap();
        let total = consumer.join().unwrap();
        assert_eq!(total, 0 + 1 + 2 + 3 + 4);
    }
}