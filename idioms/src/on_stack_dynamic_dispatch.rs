use std::fs;
use std::io;

#[allow(dead_code)]
fn foo_on_stack(arg: &str) -> io::Result<()> {
    // These mut live longer than `readable`, and thus are declared first:
    let (mut stdin_read, mut file_read);

    // We need to ascribe the type to get dynamic dispatch
    let _readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };

    Ok(())
}
