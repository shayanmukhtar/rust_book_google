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