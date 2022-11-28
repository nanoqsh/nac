mod lex;

fn main() {
    use {
        crate::lex::Lex,
        std::{env, fs, io},
    };

    let args: Vec<_> = env::args().skip(1).collect();
    assert!(args.len() <= 1, "too many args");

    let src = args
        .first()
        .map_or_else(|| io::read_to_string(io::stdin()), fs::read_to_string)
        .expect("read src");

    for tok in Lex::new(&src) {
        println!("{tok:?}");
    }
}
