use clipboard_win::{ formats, get_clipboard_string, set_clipboard, set_clipboard_string };
use std::io::{self, Read};
use std::process;

const USAGE: &str = "
win32yoink

Usage:
    win32yoink -o [--lf]
    win32yoink -i [--crlf] [--html]

Options:
    -o          Print clipboard contents to stdout
    -i          Set clipboard from stdin
    --lf        Replace CRLF with LF before printing to stdout
    --crlf      Replace lone LF bytes with CRLF before setting the clipboard
    --html      Set clipboard with html format
";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 1 {
        println!("{}", USAGE);
        process::exit(1);
    }

    let mut read_from_stdin = false;
    let mut write_to_stdout = false;
    let mut replace_crlf = false;
    let mut replace_lf = false;
    let mut use_html = false;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-i" => read_from_stdin = true,
            "-o" => write_to_stdout = true,
            "--lf" => replace_crlf = true,
            "--crlf" => replace_lf = true,
            "--html" => use_html = true,
            _ => {
                println!("{}", USAGE);
                process::exit(1);
            }
        }
    }

    if read_from_stdin && write_to_stdout {
        eprintln!("Error: Cannot read from stdin and write to stdout simultaneously.");
        process::exit(1);
    }

    // set default behavior
    if !read_from_stdin && !write_to_stdout {
        read_from_stdin = true;
    }

    if read_from_stdin {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

        if replace_lf {
            input = input.replace("(?<!\r)\n", "\r\n");
        }

        if use_html {
            set_clipboard_html_string(&input);
        } else {
            set_clipboard_string(&input).expect("Failed to set clipboard text");
        }

    }

    if write_to_stdout {
        let clipboard_text = get_clipboard_string().expect("Failed to get text from clipboard");

        let output = if replace_crlf {
            clipboard_text.replace("\r\n", "\n")
        } else {
            clipboard_text
        };

        println!("{}", output);

    }
}

fn set_clipboard_html_string(text: &str) {
    let html = formats::Html::new().expect("Failed to create html format");
    set_clipboard_string(&text).expect("Failed to set clipboard text");
    set_clipboard(html, &text).expect("Failed to set clipboard html");
}
