#[warn(dead_code)]
fn change(global_state: &mut u32) {
    *global_state += 1;
}

#[cfg(test)]
mod test {
    use super::change;

    #[test]
    fn test_safe_singleton() {
        let mut global_state = 0u32;

        change(&mut global_state);

        println!("Final state: {}", global_state);
    }
}
