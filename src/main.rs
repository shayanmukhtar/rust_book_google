mod arrays;
mod references;

// simple mapping of the function from arrays.rs
// note we don't need crate here
use crate::arrays::declare_array;

fn main() {
    println!("Hello, world!");

    // call the function from arrays.rs
    declare_array();

    // this one isn't mapped using the `use` word, so we have to be explicit
    arrays::declare_tuple();

    arrays::iterate_over_array();

    arrays::destructure_tuples();

    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let transpose_matrix = arrays::transpose(matrix);
    println!("{:?}", transpose_matrix);

    println!("References");
    references::example();

    references::exclusing_reference();
}
