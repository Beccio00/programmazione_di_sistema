use complex_numbers::solution::{ComplexNumber/* , ComplexNumberError*/};

// for this execise see https://doc.rust-lang.org/beta/std/primitive.f64.html
// you can find examples for all the traits that must be implemented
#[test]
pub fn test_create() {
    let a = ComplexNumber::new(1.0, 2.0);
    assert_eq!(a.real(), 1.0);
    assert_eq!(a.imag(), 2.0);
}

#[test]
pub fn test_create_from_real() {
    let a = ComplexNumber::from_real(10.0);
    assert_eq!(a.real(), 10.0);
    assert_eq!(a.imag(), 0.0);
}

#[test]
pub fn test_display() {
    // let's have a nice print for our complex numbers and not the ugly debug
    let a: ComplexNumber = ComplexNumber::new(4.0, 2.0);
    let b: ComplexNumber = ComplexNumber::new(4.2, 4.2);
    let c: ComplexNumber = ComplexNumber::new(4.2, -4.2);
    assert_eq!(format!("{a}"), "4 + 2i");
    assert_eq!(format!("{b}"), "4.2 + 4.2i");
    assert_eq!(format!("{c}"), "4.2 - 4.2i");
}

/*
#[test]
pub fn test_add() {
    // implement Add trait
    // remember to set: type Output = Self;
    // see: https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#add--addassign

    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = a + b;

    assert_eq!(c.to_tuple(), (2.0, 4.0));
}

#[test]
pub fn test_add_with_real() {
    // set RHS (rihgt hand side) type for Add!!! It's default value is Self, but it can be changed to anything
    let a = ComplexNumber::new(1.0, 2.0);
    let b = a + 10.0;

    assert_eq!(b.to_tuple(), (11.0, 2.0))
}

#[test]
pub fn test_inc_add() {
    let mut a = ComplexNumber::new(1.0, 2.0);
    a += ComplexNumber::new(2.0, 4.0);

    assert_eq!(a.to_tuple(), (3.0, 6.0))
}

#[test]
pub fn test_add_with_reference() {
    // references for Rust are new types: you must define the trait for them as RHS
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = a + &b;

    assert_eq!(c.to_tuple(), (2.0, 4.0))
}

#[test]
pub fn test_add_reference_with_reference() {
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = &a + &b;

    assert_eq!(c.to_tuple(), (2.0, 4.0))
}

#[test]
pub fn test_enable_copy() {
    // why this code won't compile? Read carefully the error message
    // what do we nee to do to make it work?
    let a = ComplexNumber::new(1.0, 2.0);

    let b = a + a;

    assert_eq!(b.to_tuple(), (2.0, 4.0));
}

#[test]
pub fn test_default_values() {
    // If we want to create an array of complex numbers we need to initialize values with something
    // Arrays can't be not initialized in Rust
    let array: [ComplexNumber; 10] = Default::default();

    for el in array.as_slice() {
        assert_eq!(el.to_tuple(), (0.0, 0.0));
    }
}

// commented out again when implementing TryInto see note below
//#[test]
//pub fn test_convert_into_real() {
//    let a = ComplexNumber::from_real(1.0);
//    let b: f64 = a.into();
//
//    assert_eq!(b, 1.0);
//
//}

// commented out again when implementing TryInto because it's covered by TryInto see note below
//#[test]
//pub fn test_panic_when_impossible_to_convert_to_real() {
//    // we can convert into a real only if imag is 0
//    let a = ComplexNumber::new(1.0, 2.0);
//
//    let result = std::panic::catch_unwind(|| {
//        let _: f64 = a.into();
//    });
//
//    assert!(result.is_err());
//}

#[test]
pub fn test_try_into_f64() {
    // write trait and a test for the Trait TryInto for converting into f64
    // the test must check both success and error conditions

    // Warning: when implementing this trait you will get a compilation error if you don't delete the
    // Into Trait.
    // Why? Because the std lib has a default implementation for TryInto for all types that implement Into.
    // (You can try to write it yourself, it's trivial)
    // How do we solve this? We delete the Into implementation and the above tests using Into
    // The purpose is that if the conversion may fail, then you are encouraged to write only TryInto, 
    // and we are not allowed to use Into.
    // Instead if we have Into the implementation of TryInto is trivial

    let a = ComplexNumber::new(1.0, 2.0);
    if let Err(e) = TryInto::<f64>::try_into(a) {
        assert_eq!(e, ComplexNumberError::ImaginaryNotZero);
    } else {
        assert!(false);
    }

    let b = ComplexNumber::new(1.0, 0.0);
    if let Ok(v) = TryInto::<f64>::try_into(b) {
        assert_eq!(v, 1.0);
    } else {
        assert!(false);
    }
}

#[test]
pub fn test_try_from_f64() {
    // write a trait allowing let complex = f64.into()
    // and write test

    let a: ComplexNumber = 1.0.into();
    assert_eq!(a.to_tuple(), (1.0, 0.0));
}

#[test]
pub fn test_comparison() {
    let c = ComplexNumber::new(3.0, 6.0);
    let mut v = vec![
        ComplexNumber::new(1.0, 2.0),
        ComplexNumber::new(2.0, 4.0),
        c,
    ];

    v.retain(|el| *el == c);

    assert_eq!(v.len(), 1);
}

#[test]
pub fn test_sorting() {

    // In order to use our type with generic alghoritms of the std lib, like sort(),
    // we need to implement other Traits, like Ord.

    // for sorting we can use the modulus of a complex number
    //https://www.cuemath.com/algebra/modulus-of-complex-number/
    // if |a| > |b| than a > b

    // Be careful: sort requires Ord and f64 does not implement Ord since NaN != NaN and you can't
    // use cmp from f64 to implement Ord for ComplexNumber

    // However f64 has total_cmp which produces total ordering and you can use it 
    // to implement Ord for ComplexNumber
    // https://doc.rust-lang.org/beta/std/primitive.f64.html#method.total_cmp
    
    // Other option is to use < > explicitly in the cmp method

    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(2.0, 4.0);
    let c = ComplexNumber::new(3.0, 6.0);
    let mut v = vec![c, b, a];

    v.sort();

    assert_eq!(v[0], a);
    assert_eq!(v[1], b);
    assert_eq!(v[2], c);
}

#[test]
pub fn test_as_ref() {
    // implement AsRef<f64> for ComplexNumber
    // allow a ref to real part as &f64

    let a = ComplexNumber::new(1.0, 2.0);
    let r = a.as_ref();

    assert_eq!(*r, 1.0);
}

#[test]
pub fn test_as_mut() {
    // implement AsMut<f64> for ComplexNumber
    // allow a mutable ref to real part as &mut f64

    let mut a = ComplexNumber::new(1.0, 2.0);
    let r = a.as_mut();

    *r = 10.0;

    assert_eq!(a.real(), 10.0);
}

#[test]
pub fn test_hash_with_hash_map() {
    // in order to use complex numbers in a hash map we need to implement the Hash Trait
    // https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#hash
    // The Hash trait, calling the hash method, supplies an efficient Hasher; 
    // the implementation must feed the hasher with the bits to be hashed.
    // We can use the to_bits method of f64 to get a u64 stoting the raw bits of the float,
    // and then we may write them into the hasher
    
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(2.0, 4.0);
    let c: ComplexNumber = 3.0.into();

    let mut map = std::collections::HashMap::new();

    // first insert must return None: not present
    match map.insert(a, b) {
        None => assert!(true),
        Some(_) => assert!(false),
    };

    // trty ro replace value with c
    match map.insert(a, c) {
        None => assert!(false),
        Some(x) => assert_eq!(x.to_tuple(), (2.0, 4.0)), // should return the old value, b
    };
}

#[test]
pub fn test_deque() {
    // implement VecDeque for ComplexNumber
    // 1. create a VecDeque with capacity 10
    // 2. push 10 values in the deque
    // 4. find the index of a value with binary_search: it works only if the deque is sorted!!!
    // 5. check the result: it should be meaningless
    // 3. sort the deque and check again the result of binary_search, now it should be correct

    use std::collections::VecDeque;
    let mut q = VecDeque::<ComplexNumber>::with_capacity(10);
    for i in (0..10).rev() {
        q.push_back(ComplexNumber::new(i as f64, i as f64));
    }

    // if not sorted the result is meaningless
    match q.binary_search(&ComplexNumber::new(2.0, 2.0)) {
        Ok(idx) => {
            // ...either it may find and index, but this could fail randomly
            // assert_neq!(q[idx].to_tuple(), (2.0, 2.0));
            
            // we can just assert there is an index <10
            assert!(idx < 10);
        }
        Err(_) => {
            // ...or it could miss the value and return an Err
            assert!(true)
        },
    }

    q.make_contiguous().sort();

    // once contiguous and sorted we can safely use binary_search
    if let Ok(idx) = q.binary_search(&ComplexNumber::new(2.0, 2.0)) {
        assert_eq!(q[idx].to_tuple(), (2.0, 2.0));
    } else {
        assert!(false);
    }
}
 */