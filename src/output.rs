fn get_output_file() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("lex.yy.c")
        .unwrap();
}

pub fn print_partA(text: &str) {
    let mut output = File::create(path)?;
}