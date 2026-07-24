fn hi() {}

static FN_POINTERS: &[fn()] = &[
    hi,
];

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "expected exactly one filename");

    let file = std::path::Path::new(&args[0]);

    if !file.ends_with(".mfwwg") {
        panic!("The file ending must be .mfwwg!")
    }

    println!("{:?}", file);
}
