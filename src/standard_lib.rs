// the rust standard library contains two essential components
// core are the core data types and a few methods that require 
// no heap allocation, and can work even in the absence of an 
// operating system. alloc contains all the types that do require
// a heap allocator, like Vec, Box, Arc, and others

// Also something to note, in rust, we document using markdown in the 
// actual code itself, marked with /// at the beginning

use std::fs::File;

/// # This is markdown header title 1
/// We can write some stuff here, like a description of the function
/// ## There can be heading 2
/// We can even embed code and unit tests in the documentation 
pub fn is_divisible_by(left: u32, right: u32) -> bool {
    if right == 0 {
        return false
    }
    left % right == 0
}

// note - published rust crates automatically create a docs.rs file that 
// contains all the documents generated from your source code

// The most common one that we've seen is Option<T>
// It Some(T) or None - this can often be a zero cost abstraction, because
// the compiler will automatically try to find a naturally invalid value for
// example, if we use Option<&Foo>, then we can just use a null ptr to 
// represent None

// Result<T, E> is the next one, where we've already seen plenty of match statements
// on it, as it either returns Ok(T) or Err(e) - we try to use Result<T, E> as much
// as possible, because the result is embedded in the return type, it encourages good
// error handling. Where we never expect an error, we can use .unwrap on the return. 
// This call, however, will panic if something is not expected.

// there is special syntax built for the Result<T, E> type
// take a look at the following example:
struct FileReplace {
    name: String
}

impl FileReplace {
    pub fn write_all(&self, input: String) -> Result<(), String> {
        Ok(())
    }
}

pub fn the_dirty_way_result() -> Result<(), String> {
    let mut file: FileReplace = FileReplace{name: "filename.txt".to_string()};

    // the cascaded if statements that are there to avoid the nested ifs by 
    // checking for error on each operation
    if let Err(e) = file.write_all("string".to_string()) {
        return Err(e)
    }
    if let Err(e) = file.write_all("other_string".to_string()) {
        return Err(e)
    }
    if let Err(e) = file.write_all("one_more".to_string()) {
        return Err(e)
    }

    Ok(())
}

// this whole thing can be replaced by this:
pub fn the_clean_way() -> Result<(), String> {
    let mut file: FileReplace = FileReplace{name: "filename.txt".to_string()};

    // this question mark syntax replaced the whole if let Err(e) stuff - it basically
    // tells the compiler that if the Result is Ok, go forward, otherwise just return
    // from the function the Err(e) that is inside Result
    file.write_all("string".to_string())?;
    file.write_all("other_string".to_string())?;
    file.write_all("last_string".to_string())?;

    Ok(())
}

