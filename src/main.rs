use std::{io::Write, time::Duration};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use humantime::format_duration;
use regex::Regex;

lazy_static::lazy_static! {
	static ref ALHPANUMERIC_SPLIT_RE: Regex = Regex::new(r"(\d+)(\D+)").unwrap();
}

fn pl(string: &str, count: usize) -> String {
    match count {
        1 => string.to_string(),
        _ => format!("{}s", string),
    }
}

#[derive(Debug, Clone)]
struct GitStatus {
	emoji: char,
	filename: String,
	ago_raw: Duration,
	ago: [String; 2],
	colour: Color,
	msg: String
}

fn main() {
	let code_dir = std::env::args().nth(1).unwrap();

	let mut entries: Vec<GitStatus> = std::fs::read_dir(code_dir).unwrap().filter_map(|entry| {
		let entry = entry.unwrap();
		let path = &entry.path();

		let filename = entry.file_name();
		let filename = filename.to_str().unwrap().to_string();

		let ago_raw = entry
			.metadata()
			.unwrap()
			.modified()
			.unwrap()
			.elapsed()
			.unwrap();

		let formatted_modified: [String; 2] = ALHPANUMERIC_SPLIT_RE
			.captures(&format_duration(ago_raw)
			.to_string()
			.split_whitespace()
			.next()
			.unwrap()
			.to_string()
		).map(|x| [x.get(1).unwrap().as_str().to_string(), x.get(2).unwrap().as_str().to_string()]).unwrap();

		if path.is_file() {
			return Some(GitStatus {
				emoji: '📝',
				filename,
				ago_raw,
				ago: formatted_modified,
				colour: Color::Blue,
				msg: String::from("loose file; remember to back up")
			})
		}

		let cherry = std::process::Command::new("git")
			.args(["cherry", "-v"])
			.current_dir(&path)
			.output()
			.unwrap();

		dbg!(cherry);

		let git_status_output = std::process::Command::new("git")
			.args(["status", "-u", "--porcelain"])
			.current_dir(&path)
			.output();

		match git_status_output {
            Ok(o) => {
                if o.status.success() {
                    let out = String::from_utf8_lossy(&o.stdout);

					let lines = out.trim().split("\n");
					let untracked_count = lines.clone().filter(|l| l.starts_with("?")).count();
					let modified_count = lines.filter(|l| l.starts_with("M")).count();

					if untracked_count == 0 && modified_count == 0 {
						return Some(GitStatus {
							emoji: '✅',
							filename,
							ago_raw,
							ago: formatted_modified,
							colour: Color::Green,
							msg: String::from("is clean")
						})
					} else {
						let mut message = String::new();

						if untracked_count > 0 {
							message.push_str(&format!("{} {}", untracked_count, pl("untracked change", untracked_count)));
						}

						if untracked_count > 0 && modified_count > 0 {
							message.push_str(" and ");
						}

						if modified_count > 0 {
							message.push_str(&format!("{} {}", modified_count, pl("modification", modified_count)));
						}

						return Some(GitStatus {
							emoji: '📂',
							filename,
							ago_raw,
							ago: formatted_modified,
							colour: Color::Yellow,
							msg: message
						});
					}
                } else {
                    let out = String::from_utf8_lossy(&o.stderr);

					if out.to_string().contains("not a git repository") {
						return Some(GitStatus {
							emoji: '❌',
							filename,
							ago_raw,
							ago: formatted_modified,
							colour: Color::Red,
							msg: String::from("no git repository")
						});
					} else {
						return Some(GitStatus {
							emoji: '❗',
							filename,
							ago_raw,
							ago: formatted_modified,
							colour: Color::Red,
							msg: String::from("there was a problem with git")
						});
					}
                }
            }
            Err(_) => None,
        }
	}).collect();
	entries.sort_by(|a, b| a.ago_raw.cmp(&b.ago_raw));

	let mut max_filename_len = 0;
	let mut max_ago_len = 0;
	let mut max_msg_len = 0;
	for entry in entries.clone().into_iter() {
		let ago_len = entry.ago.join("").len();

		if entry.filename.len() > max_filename_len { max_filename_len = entry.filename.len() }
		if ago_len > max_ago_len { max_ago_len = ago_len }
		if entry.msg.len() > max_msg_len { max_msg_len = entry.msg.len() }
	}

	let mut stdout = StandardStream::stdout(ColorChoice::Always);
	let seg_margin = 1;

	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true)).unwrap();
	write!(&mut stdout, "{:<1$}", '❔', seg_margin + 1).unwrap();
	write!(&mut stdout, "{:<1$}", "Directory", max_filename_len + seg_margin).unwrap();
	write!(&mut stdout, "{:<1$}", "Last modified", max_ago_len).unwrap();
	writeln!(&mut stdout, "{:>1$}", "Status", max_msg_len + seg_margin - 1).unwrap();

	for entry in entries.into_iter() {
		write!(&mut stdout, "{:<1$}", entry.emoji, seg_margin + 1).unwrap();

		stdout.set_color(ColorSpec::new().set_fg(Some(entry.colour))).unwrap();
		write!(&mut stdout, "{:<1$}", entry.filename, max_filename_len + seg_margin).unwrap();

		stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta))).unwrap();
		write!(&mut stdout, "{:>2} {:<2$}ago", entry.ago[0], entry.ago[1], max_ago_len).unwrap();

		stdout.set_color(ColorSpec::new().set_fg(Some(entry.colour))).unwrap();
		writeln!(&mut stdout, "{:>1$}", entry.msg, max_msg_len + seg_margin).unwrap();
	}
}
