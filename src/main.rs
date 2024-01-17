use anyhow;
use std::sync::{Arc, Mutex};
use tree_sitter as ts;
use tree_sitter_nix as nix;

mod lamentation;
mod types;
mod util;

fn main() -> anyhow::Result<()> {
    let mut workers = vec![];
    for x in lamentation::MODULES {
        workers.push((x.new)()?);
    }

    // Fold iterator into the vector so the data shared by the threads
    // is just an index.
    let args: Vec<_> = std::env::args().skip(1).collect();

    let index = Arc::new(Mutex::new(0));
    let errcode = Arc::new(Mutex::new(0)); // doubles as stdout mutex
    let args = Arc::new(args);
    let workers = Arc::new(workers);

    let mut threads = vec![];
    for _ in 0..8 {
        let index = Arc::clone(&index);
        let args = Arc::clone(&args);
        let errcode = Arc::clone(&errcode);
        let workers = Arc::clone(&workers);

        let thread = std::thread::spawn(move || {
            let language = nix::language();
            let mut parser = ts::Parser::new();
            parser.set_language(language).unwrap();

            loop {
                let mut index = index.lock().unwrap();
                if *index == args.len() {
                    break;
                }
                let fname = &args[*index];
                *index += 1;
                drop(index);

                match std::fs::read(fname) {
                    Err(e) => {
                        let mut errcode = errcode.lock().unwrap();
                        *errcode = 1;
                        eprintln!("Failed to read `{}': {:?}", fname, e);
                    }
                    Ok(content) => {
                        // None is only returned on misuse or timeout.
                        let tree = parser.parse(&content, None).unwrap();

                        let mut laments = vec![];
                        for w in workers.iter() {
                            laments.extend(w.lament(&tree, &content));
                        }

                        if laments.len() > 0 {
                            let mut errcode = errcode.lock().unwrap();
                            *errcode = 1;
                            for e in laments {
                                println!(
                                    "{}:{}:{}: {:?} {}",
                                    &fname, e.line, e.column, e.kind, e.message
                                );
                            }
                        }
                    }
                }
            }
        });
        threads.push(thread);
    }
    for t in threads {
        t.join().unwrap();
    }

    Ok(())
}
