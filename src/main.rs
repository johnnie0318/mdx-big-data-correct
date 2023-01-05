use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String
}

impl <R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines{reader: io::BufReader::new(r), buf: String::new()}
    }
    fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>>{
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(nbytes) => if nbytes == 0 {
                None // no more lines!
            } else {
                let line = self.buf.trim_right();
                Some(Ok(line))
            },
            Err(e) => Some(Err(e))
        }
    }
}

fn correct_json_file(input_name: &str, output_name: &str) -> io::Result<()> {
    let file = File::open(&input_name)?;
    let mut out = File::create(output_name)?;

    let mut lines = Lines::new(file);
    while let Some(line) = lines.next() {
        let line = line?;
        println!("{}", line.len());
        write!(out, "{}", &line.replace(";", "':'"))?;
    }
    Ok(())
}
// fn read_to_string(filename: &str) -> Result<String,io::Error> {
//     let mut file = File::open(&filename)?;
//     let mut text = String::new();
//     file.read_to_string(&mut text)?;
//     Ok(text)
// }

fn main() {
    let input_file = env::args().nth(1).expect("please supply input filename");
    let output_file = env::args().nth(2).expect("please supply output filename");

    println!("Input File: {}\nOutput File: {}", input_file, output_file);

    // let text = read_to_string(&file).expect("bad file man!");
    // println!("file had {} bytes", text.len());
    // read_all_lines(&file);

    correct_json_file(&input_file, &output_file);
}