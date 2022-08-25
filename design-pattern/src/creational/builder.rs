#[derive(Debug, PartialEq)]
pub struct Foo {
    // Lots of complicated fields.
    bar: String,
}

impl Foo {
    // This method will help users to discover the builder
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    // Probably lots of optional fields.
    bar: String,
}

impl FooBuilder {
    pub fn new() -> FooBuilder {
        FooBuilder {
            bar: String::from("X"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        // Set the name on the builder itself, and return the builder by balue.
        self.bar = bar;
        self
    }

    // If we can get away with not consuming the Builder here, that is an
    // advantage. It means we can use the FooBuilder as a template for constructing
    // many Foos.
    pub fn build(self) -> Foo {
        // Create a Foo from the FooBuilder, applying all settings in FooBuilder
        // to Foo
        Foo { bar: self.bar }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test() {
        let foo = Foo {
            bar: String::from("Y"),
        };
        let foo_from_builder: Foo = FooBuilder::new().name(String::from("Y")).build();

        assert_eq!(foo, foo_from_builder);
    }
}
