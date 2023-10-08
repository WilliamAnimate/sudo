// this project (sudo) is literally just a shell for MinSudo. it does nothing
use std::{process::Command, time::Duration, thread::sleep};

fn main() {
	let mut is_error: bool = false;

	// MinSudo.exe --NoLogo --TrustedInstaller cmd.exe /k
	// "MinSudo.exe" --NoLogo --TrustedInstaller "cmd.exe /k"
	match Command::new("MinSudo.exe").args(["--NoLogo", "--TrustedInstaller", "--Privileged", "cmd.exe", "/k"]).spawn() {
		Ok(_) => {/* 👌 */},
		Err(err) => {
			eprintln!("Failed to launch MinSudo: {}", err);
			is_error = true;
		},
	}

	if is_error {
		println!("program experienced an error, hit control + c to exit.");
		loop {
			sleep(Duration::from_secs(69));
		}
	}

	std::process::exit(0);
}