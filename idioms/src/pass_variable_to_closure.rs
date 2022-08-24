use std::rc::Rc;

#[allow(dead_code)]
fn foo() {
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);

    let closure = {
        let num2 = num2.clone();
        let num3 = num3.clone();

        move || {
            *num1 + *num2 + *num3;
        }
    };
}
