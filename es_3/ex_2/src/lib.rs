pub mod solution {
    use std::collections::VecDeque;
    use std::path::Display;
    use std::fmt;
    use std::ops::{Add, AddAssign};
    use std::convert::{TryInto, Into};
    use std::cmp::{Ord, Ordering, PartialOrd};
    use std::hash::{Hasher, Hash};

    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError {
       ImaginaryNotZero,
   }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ComplexNumber {
        real: f64,
        imag: f64,
    }

    impl ComplexNumber {
        pub fn new(r: f64, i: f64) -> ComplexNumber {
            ComplexNumber{real: r, imag: i}
        }

        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }

        pub fn from_real(r: f64) -> ComplexNumber {
            ComplexNumber{real: r, imag: 0.0}
        }

        pub fn to_tuple(&self) -> (f64, f64)  {
            (self.real, self.imag)
        }

        pub fn get_modulus(&self) -> f64 {
            let distance = self.real*self.real + self.imag*self.imag;
            distance.sqrt()
        }


    
    }

    impl fmt::Display for ComplexNumber {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.imag >= 0.0 {
                return write!(f, "{} + {}i", self.real, self.imag);
            } else {
                return write!(f, "{} - {}i", self.real, self.imag.abs());
            }

        }
    }

    impl Add for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: ComplexNumber) -> ComplexNumber {
            
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Add<f64> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: f64) -> Self::Output {
            ComplexNumber {
                real: self.real + rhs,
                imag: self.imag,
            }
        }
    }

    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
            
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Add<&ComplexNumber> for &ComplexNumber {
        type Output = ComplexNumber;

        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,

            }
        }
    }

    impl AddAssign for ComplexNumber {
        fn add_assign(&mut self, rhs: Self) {
            self.real += rhs.real;
            self.imag += rhs.imag;    
        }
    }

    impl Default for ComplexNumber {
        fn default() -> Self {
            ComplexNumber { real: 0.0, imag: 0.0}
        }
    }
    
    // impl Into<f64> for ComplexNumber {
    //     fn into(self) -> f64 {
    //         if self.imag == 0.0 {
    //             return self.real;
    //         } else {
    //             panic!("Error: impossible to convert")
    //         }
    //     }
    // }
 

    impl TryInto<f64> for ComplexNumber {
        type Error = ComplexNumberError;

        fn try_into(self) -> Result<f64, ComplexNumberError> {
            match self.imag {
                0.0 => {Ok(self.real)}
                _ => {Err(ComplexNumberError::ImaginaryNotZero)}
            }
        }
    }

    impl Into<ComplexNumber> for f64 {
        fn into(self) -> ComplexNumber {
            ComplexNumber { real: self, imag: 0.0 }
        }
    }

    impl PartialOrd for ComplexNumber{
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.get_modulus().partial_cmp(&other.get_modulus())
        }
    }

    impl Eq for ComplexNumber {}

    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.get_modulus().total_cmp(&other.get_modulus())
        }
    } 

    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.real
        }   
    }

    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_i64(self.real as i64);
            state.write_i64(self.imag as i64);
        }
    }

    

}
