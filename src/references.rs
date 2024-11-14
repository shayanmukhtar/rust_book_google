use core::slice;
use std::arch::x86_64;

pub fn example() {
    // both of these are of type char
    let a = 'A';
    let b = 'B';

    // r is now of type &char - ie. a reference to a char
    // and we make it equal to the reference of a (same as 
    // the address of a in c lang). 
    let mut r: &char = &a;

    // we can print the value of *r
    // just like C, we use the `*` to dereference the reference
    println!("{}", *r);

    // because r is a mutable reference, we can change what r references
    r = &b;
    println!("{}", *r);

    // we never need to do a null check on a reference, the compiler will not
    // allow it to be null
    // rust will never make a reference for you - the `&` is required. However,
    // rust will do the dereference for you when you use the `.` operator
    // i.e. the following will access the underlying value referenced by r
    let is_ascii = r.is_ascii();
    println!("{}", is_ascii);

    // references are also shared only, so you can't modify what the reference 
    // points to
    // *r = 'C'; // this will fail to compile
}

pub fn exclusing_reference() {
    // we can have an exclusive reference by using `&mut`
    // of course the underlying value itself must also be mutable
    let mut point = (10, 50);

    // let's make an exclusive reference to a mutable value
    //let x_coord = &mut point.0;
    // see how this is different than a mutable reference, which we 
    // declare is `let mut x_coord = &point.0;`
    // let mut x_coord = &point.0; is a shared reference to a mutable value,
    // you cannot change the value of point.0 through x_coord
    let mut x_coord = & point.0;
    println!("{}", *x_coord);
    //*x_coord = 20;  // this fails to compile

    // but now if we make an exclusive reference to the value
    let x_coord = &mut point.0;

    // now we can change the value of point.0 through x_coord
    *x_coord = 20;
    println!("{}", *x_coord);

    // but now, since we have an exclusive reference to point.0, nothing else
    // should be allowed to modify it
    //point.0 = 30; // this fails to compile because on the next line, we again reference point.0
                    // through x_coord - this is typically allowed in C/C++, but is forbidden in Rust

    println!("{:?}", *x_coord);
}

pub fn slices() {
    let mut data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // slice is a reference to a slice, notice that its type no longer contains 
    // the size of the array
    let slice: &[i32] = &data[1..4];

    // we can print the slice
    println!("{:?}", slice);

    // we can also iterate over the slice
    for i in slice {
        println!("{}", i);
    }

    // since slice is a reference to an array, we cannot let data go out of scope

    // we can also start from the beginning of an array, or to the end
    let slice: &[i32] = &data[..4];
    println!("{:?}", slice);

    let slice: &[i32] = &data[4..];
    println!("{:?}", slice);

    // the main takeway here is in rust, the reference is a pointer, and it does not
    // own the object. An owned object is the object where the memory actually lives.
    // references don't own the memory, they just point to it.
}

pub fn strings() {
    // two keys here - there is &str, which is a reference to an owned value
    // of a string, similar to &[u8]. Then there is String, which is the owned
    // value containing string data, basically a Vec<u8>
    
    // for example, over here s_ref is a reference to a string, the owned
    // string data is stored in the binary of the program (its const)
    let s_ref: &str = "world!";

    // s_owned is a String, which is a heap allocated string
    let mut s_owned: String = String::from("Hello, ");

    // we can append the string to the end, since its mutable data
    s_owned.push_str(s_ref);

    println!("s_owned: {}", s_owned);

    // we can also declare a slice to take a portion of the string
    let slice: &str = &s_owned[&s_owned.len() - s_ref.len()..];
    println!("slice: {}", slice);
}


// notice in the function signatures we use &[f64; 3], and not just &[f64].
// That's because we know and therefore want to enforce that the slice has exactly 3 elements.
// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
pub fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut sum: f64 = 0.0;
    for i in vector {
        sum += i * i;
    }
    sum.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


pub fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for i in vector {
        *i /= mag;
    }
}