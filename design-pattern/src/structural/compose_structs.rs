struct A {
    b: B,
    c: C,
}

struct B {
    f2: u32,
}

struct C {
    f1: u32,
    f3: u32,
}

#[allow(dead_code)]
fn foo(b: &mut B) -> &u32 {
    &b.f2
}

#[allow(dead_code)]
fn bar(c: &mut C) -> u32 {
    c.f1 + c.f3
}

#[allow(dead_code)]
fn baz(a: &mut A) {
    // The later usage of x causes a to be borrowed for the rest of the function.
    let x = foo(&mut a.b);
    // Borrow checker error;
    let _y = bar(&mut a.c);
    println!("{}", x);
}
