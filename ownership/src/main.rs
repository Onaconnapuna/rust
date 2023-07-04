// Ownership 

// Stack and Heap
// All data stored on the stack must have a known, fixed size. 
// Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

// The heap is less organized: when you put data on the heap, you request a certain amount of space. 
// The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, 
// which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating
//  (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, 
// fixed size, you can store the pointer on the stack, 
// but when you want the actual data, you must follow the pointer

// As we can infer, using the heap is slower than the stack

// Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, 
// and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. 

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main {

        {                      // s is not valid here, it’s not yet declared
            let s = "hello";   // s is valid from this point forward

            // do stuff with s
        }                      // this scope is now over, and s is no longer valid
    
}

fn testing_repo {

    {
        let random_block = 10;
    }

}