use lazy_static::lazy_static;
use std::sync::Mutex;

#[warn(dead_code)]
fn change(global_state: &mut u32) {
    *global_state += 1;
}

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

#[cfg(test)]
mod test {
    use super::{change, do_a_call, ARRAY};

    #[test]
    fn test_safe_singleton() {
        let mut global_state = 0u32;

        change(&mut global_state);

        println!("Final state: {}", global_state);
    }

    #[test]
    fn test_lazy_singleton() {
        do_a_call();
        do_a_call();
        do_a_call();

        println!("Called {}", ARRAY.lock().unwrap().len());
    }
}
