use std::mem;

#[allow(dead_code)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

#[allow(dead_code)]
fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will be assigned to `*e`)
        *e = MyEnum::B {
            name: mem::take(name),
        }
    }
}

enum MultiVariantEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

#[allow(dead_code)]
fn swizzle(e: &mut MultiVariantEnum) {
    *e = match e {
        // Ownership rules do not allow `name` by value, but we cannot
        // take the value out of multiple reference, unless we replace it:
        MultiVariantEnum::A { name } => MultiVariantEnum::B {
            name: mem::take(name),
        },
        MultiVariantEnum::B { name } => MultiVariantEnum::A {
            name: mem::take(name),
        },
        MultiVariantEnum::C => MultiVariantEnum::D,
        MultiVariantEnum::D => MultiVariantEnum::C,
    }
}
