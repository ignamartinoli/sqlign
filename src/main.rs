use std::fs::File;
use std::io::{self, prelude::*, Write};

use clap::Parser;
use tree_sitter::Node;

#[derive(Parser)]
#[command(about = "This utility formats SQL code alignment", bin_name = "sqlign")]
struct Args {
	#[arg(long = "input", short = 'i')]
	input: std::path::PathBuf,

	#[arg(long = "output", short = 'o')]
	output: std::path::PathBuf
}

fn main() {
	if let Err(e) = run() {
		eprintln!("Error: {}", e);
		std::process::exit(1)
	}
}

fn run() -> Result<(), io::Error> {
	let args = Args::parse();
	let mut input = File::open(&args.input)?;
	let mut output = File::create(&args.output)?;

	let mut source_code = String::new();
	input.read_to_string(&mut source_code)?;

	let mut parser = tree_sitter::Parser::new();
	parser
		.set_language(tree_sitter_sql::language())
		.expect("Failed to load SQL language");
	let tree = parser.parse(&source_code, None).unwrap();

	let root_node = tree.root_node();

	// list_nodes(&root_node, &source_code, 0);

	run_source(&root_node, &source_code, &mut output)?;

	Ok(())
}

fn run_source(node: &Node, source: &str, file: &mut File) -> Result<(), io::Error> {
	for child in node.children(&mut node.walk()) {
		if child.kind().ends_with("statement") {
			run_statement(&child, source, file, ";\n")?;
		}
	}

	Ok(())
}

fn run_statement(
	node: &Node,
	source: &str,
	file: &mut File,
	ending: &str
) -> Result<(), io::Error> {
	let longest = node
		.children(&mut node.walk())
		.map(|child| child.kind().len())
		.max()
		.unwrap_or(0);

	for (i, child) in node.children(&mut node.walk()).enumerate() {
		let indent = longest.saturating_sub(child.kind().len());
		write!(file, "{}", " ".repeat(indent))?;

		run_clause(&child, source, file)?;

		let ending = if i + 1 == node.child_count() { ";" } else { "" };
		write!(file, "{ending}\n")?;
	}

	Ok(())
}

fn run_clause(node: &Node, source: &str, file: &mut File) -> Result<(), io::Error> {
	for (i, child) in node.children(&mut node.walk()).enumerate() {
		if child.child_count() == 0 {
			write!(file, "{}", child.utf8_text(source.as_bytes()).unwrap())?;
		} else if child.kind() == "dotted_name" {
			run_dotted_name(&child, source, file)?;
		} else {
			run_clause(&child, source, file)?;
		}

		let ending = if i + 1 == node.child_count() { "" } else { " " };
		write!(file, "{}", ending)?;
	}

	Ok(())
}

fn run_dotted_name(node: &Node, source: &str, file: &mut File) -> Result<(), io::Error> {
	for child in node.children(&mut node.walk()) {
		if child.child_count() == 0 {
			write!(file, "{}", child.utf8_text(source.as_bytes()).unwrap())?;
		} else {
			run_dotted_name(&child, source, file)?;
		}
	}

	Ok(())
}

fn list_nodes(node: &Node, source_code: &str, mut level: u8) {
	level += 1;
	for child in node.children(&mut node.walk()) {
		println!(
			"-[ {}. Node {} ]-)\n{}",
			level,
			child.kind(),
			child.utf8_text(source_code.as_bytes()).unwrap()
		);

		list_nodes(&child, source_code, level)
	}
}
