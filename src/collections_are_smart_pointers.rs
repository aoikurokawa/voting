use std::ops::Deref;

// The Box<T> smart pointer provides a way to reference data stored in the heap memory.
// By default, Rust allocates everything on the stack memory.
// Using the box smart pointer, we can access data on the heap, incurring no performance overhead
// Once the program is terminated, the box is deallocated from the memory
#[allow(dead_code)]
fn foo_box() {
    let var = Box::new(100);
    println!("Value: {}", var);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[allow(dead_code)]
fn foo_list() {
    let _list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

// Deref<T> smart pointer is used to allow manipulation of the dereferencing operation
// Using the deref smart pointer, you can create a struct that can work with both smart pointer and references
struct CustBox<T> {
    data: T,
}

impl<T> Deref for CustBox<T> {
    // generic type param
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

#[allow(dead_code)]
fn foo_deref() {
    let x = CustBox { data: 100 };
    println!("Value: {}", *(x.deref()))
}

// Drop smart pointer is used to free memory that is allocated in the heap.
// The Drop trait in Rust handles free the memory when a variable goes out of scope
// We do not need to call this trait manually, as Rust will do this for us.
// However, we can implement it for custom types as shown
struct MyStruct {
    x: i32,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {}", self.x)
    }
}

#[allow(dead_code)]
fn foo_drop() {
    let _x = MyStruct { x: 100 };
    let _y = MyStruct { x: 200 };
}
