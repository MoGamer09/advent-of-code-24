use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub(crate) fn loadData() -> io::Result<()> {
    let file = File::open("./inputs/day1.txt")?; // Specify the path to your file
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}
