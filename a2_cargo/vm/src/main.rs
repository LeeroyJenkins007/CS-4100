use std::io;

fn main() -> io::Result<()>{
    let mut file_content = Vec::new();

    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])?;
    let file = BufReader::new(file);

    for line in file.lines() {
        
    }
    println!("Hello, world!");
}
