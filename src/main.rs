fn print_variable() {
    if GLOBAL_VEC.with(|v| {
        v.borrow().len() >= 1
    }) {
        GLOBAL_VEC.with(|v| {
          println!("{}", v.borrow_mut()[0]);
        });
    }
}

fn mem_leak() {
    let mut v: Vec<u64> = Vec::new();

    v.reserve(200);

    std::mem::forget(v);
}

fn change_variable() {
    if GLOBAL_VEC.with(|v| {
        v.borrow().len() >= 1
    }) {
        GLOBAL_VEC.with(|v| {
          v.borrow_mut()[0] = "mitch!".into();
        });
    }
}

fn destroy_variable() {
    if GLOBAL_VEC.with(|v| {
        v.borrow().len() >= 1
    }) {
        GLOBAL_VEC.with(|v| {
          v.borrow_mut().remove(0);
        });
    }
}

fn create_variable() {
    GLOBAL_VEC.with(|v| {
        v.borrow_mut().push("mitch".into());
    });
}

fn print_mitch() {
    println!("mitch");
}

fn print_mitch_nl() {
    print!("mitch");
}

use std::cell::RefCell;

thread_local! {
    static GLOBAL_VEC: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

static FN_POINTERS: &[fn()] = &[
    print_mitch,
    create_variable,
    change_variable,
    destroy_variable,
    print_mitch_nl,
    mem_leak, // :3
    print_variable,
];

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn count_mitch(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    const TARGET: &[u8; 5] = b"mitch";

    let mut state = 0;
    let mut count = 0;

    loop {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            break;
        }

        for &b in buf {
            if b.is_ascii_whitespace() {
                continue;
            }

            if b == TARGET[state] {
                state += 1;

                if state == TARGET.len() {
                    count += 1;
                    state = 0;
                }
            } else {
                panic!(
                    "Unexpected byte 0x{:02X} ('{}')",
                    b,
                    b.escape_ascii()
                );
            }
        }

        let len = buf.len();
        reader.consume(len);
    }

    if state != 0 {
        panic!("Incomplete 'mitch' at end of file");
    }

    Ok(count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    GLOBAL_VEC.with(|v| {
        v.borrow_mut().push("mitch".into());
    });

    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "expected exactly one filename");

    let file = std::path::Path::new(&args[0]);

    use rand::RngExt;

    let count = count_mitch(&file.to_str().unwrap())?;

    let mut rng = rand::rng();

    for _ in 0..count {
        let func = FN_POINTERS[rng.random_range(0..FN_POINTERS.len())];
        func();
    }

    Ok(())
}
