use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn write_to_file(data: &str)  -> io::Result<()> {
    let mut file = File::create("output.txt")?;
    file.write_all(data.as_bytes())?;
    file.flush()?;
    file.sync_all()?;
    Ok(())

}

fn write_to_existing_file(data: &str) -> io::Result<()> {
  let mut file = OpenOptions::new()
  .write(true)
  .append(true)
  .create(true)
  .open("output.txt")?;

  file.write_all(data.as_bytes())?;
  file.flush()?;
  file.sync_all()?;
  Ok(())
}

fn main()  -> io::Result<()> {
    let data : &str = "Hello, world!";
    write_to_existing_file(data)?;
    println!("Data written to file successfully.");
    Ok(())
}
