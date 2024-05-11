use std::io::{self, Stdin, Stdout, Write};

fn rep(stdout: &mut Stdout, stdin: &Stdin) -> io::Result<()> {
    print!("user> ");
    stdout.flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    print!("{}", buffer);

    Ok(())
}

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    loop {
        rep(&mut stdout, &stdin).unwrap();
    }
}
