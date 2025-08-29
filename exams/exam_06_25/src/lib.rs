pub mod token_acquirer{
    use std::{io::Empty, result, sync::{Condvar, Mutex}, time::Instant};

    
    type TokenAcquirer = dyn Fn() -> Result<(String, Instant), String> + Sync;
    
    #[derive(PartialEq)]
    pub enum TokenState{
        Empty,
        Pending,
        Valid,
    }


    pub struct TokenManager {
        jobs: Mutex<(Box<TokenAcquirer>, TokenState)>,
        cv: Condvar,
    }

    impl TokenManager{
        pub fn new(acquire_token: Box<TokenAcquirer> ) -> Self{
            Self { jobs: Mutex::new((acquire_token, TokenState::Empty)), cv: Condvar::new() }
        }
       
        pub fn get_token(&self) -> Result<String, String> {
            let mut guard = self.jobs.lock().unwrap();
            
            loop{
                match guard.1 {
                    TokenState::Empty => {
                        guard.1 = TokenState::Pending;
        
                        match guard.0() {
                            Ok((tok, due)) => {
                                guard.1 = TokenState::Valid;
                                self.cv.notify_all();
                                return Ok(tok);
                            }, 
                            Err(msg) => {
                                guard.1 = TokenState::Empty;
                                return Err(msg);
                            },
                        }
                    }, 

                    TokenState::Pending => {
                        guard = self.cv.wait_while(guard, |g| g.1 == TokenState::Pending).unwrap();
                    }

                    TokenState::Valid => {
                        let (tok, due) = guard.0().unwrap();
                        
                        if due > Instant::now() {
                            return Ok(tok);
                        } else {
                            guard.1 = TokenState::Pending;
                        }
                    },

                }
            }



        }

        pub fn try_get_token(&self) -> Option<String>{
            let guard = self.jobs.lock().unwrap();

            match guard.1 {
                TokenState::Valid => {
                    let (tok, due) = guard.0().unwrap();
                    if due > Instant::now() {
                        Some(tok)
                    } else {
                        None
                    }
                }

                _ => None

            }
        }
    }



}