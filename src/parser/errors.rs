
	
fn report(line: u32, stub: String, msg: String) {
	eprintln!("[line {}] Error {}: {}", line, stub, msg);
}

fn error(line: u32, msg: String) {
	report(line, String::from(""), msg);
}

