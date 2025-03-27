pub mod solution {
    use std::path::Display;
    use std::fmt;
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
    }
    impl fmt::Display for ComplexNumber {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.imag >= 0.0 {
                return write!(f, "{} + {}i", self.real, self.imag);
            } else {
                return write!(f, "{} - {}i", self.real, self.imag as u64);
            }

        }
    }


}
