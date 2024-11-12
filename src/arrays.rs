pub fn declare_array() {
    // declare an array of 5 elements - note arrays are
    // fixed size declared as [type; size]
    let first_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", first_array);

    // after declaring array of [type; size] we can 
    // instantiate all elements to the same value
    // by using [value; size]
    let mut second_array: [i32; 10] = [42; 10];
    // here we modify the 6th element of the array
    second_array[5] = 100;
    println!("{:?}", second_array);
}

pub fn declare_tuple() {
    let my_tuple: (i8, bool) = (10, true);
    // remember in println the `:?` is used to print debug info
    // and the surrounding `{}` is to say this is an fstring value
    println!("{:?}", my_tuple);

    // or you can access by index
    println!("{}", my_tuple.0);
    println!("{}", my_tuple.1);

    // the empty tuple is like `void` in c
    ()
}

pub fn iterate_over_array() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    // iterate over the array - prime is of type i32
    // this is using the trait `IntoIterator` which is implemented
    // for arrays
    for prime in primes {
        println!("{}", prime);

        // check if these are are prime numbers
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}

pub fn destructure_tuples() {
    let my_tuple: (i8, bool) = (10, true);
    // destructure the tuple into two variables
    // here we declare two local variables - number and is_it_true
    let (number, is_it_true) = my_tuple;
    println!("number: {}, is_it_true: {}", number, is_it_true);

    // we could have also done it manually
    let number = my_tuple.0;
    let is_it_true = my_tuple.1;

    println!("number: {}, is_it_true: {}", number, is_it_true);

    // the pattern itself must be irrefutable - meaning we must be 
    // able to statically match the tuple type
    // i.e. let (number, is_it_true, third) = my_tuple; wont work
    // also, if we specifically declare it should also fail
    // let (number, is_it_true) : (f32, bool) = my_tuple; // this fails
}

pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transpose_matrix : [[i32; 3]; 3] = [[0; 3]; 3];

    for row in 0..3 {
        for col in 0..3 {
            transpose_matrix[row][col] = matrix[col][row];
        }
    }

    transpose_matrix
}