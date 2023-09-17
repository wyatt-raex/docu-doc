use std::fs;

fn main() -> std::io::Result<()> {
	println!("This is a test to rename a file and/or move it.\nrenaming file...");
	fs::rename("a.txt", "b.txt")?;
	println!("\renamed file...\nmoving file...");
	fs::rename("b.txt", "folder1/c.txt")?;
	println!("moved file!\nJob Done!");
	Ok(())
}
