#[allow(dead_code)]
fn foo() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);

    // equivalent to
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }
}

#[allow(dead_code)]
fn foo1() {
    let turing = Some("Turing");
    let logicians = vec!["Curry", "Kleene", "Markov"];

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }
}
