use std::fs::File;
use std::io::{self, prelude::*, Write};
use std::result::Result;

use clap::Parser;
use tree_sitter::{Node, TreeCursor};

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

	list_nodes(&root_node, &source_code, 0);

	// run_nodes(&root_node, &source_code, &mut output, "", "")?;

	run_source(&root_node, &source_code, &mut output)?;

	Ok(())
}

fn run_source(node: &Node, source: &str, file: &mut File) -> Result<(), io::Error> {
	for child in node.children(&mut node.walk()) {
		if child.kind().ends_with("statement") {
			let longest = child
				.children(&mut node.walk())
				.map(|child| child.kind().len())
				.max()
				.unwrap_or(0);

			run_statement(&child, source, file, longest)?;
			write!(file, ";\n")?;
		}
	}

	Ok(())
}

fn run_statement(
	node: &Node,
	source: &str,
	file: &mut File,
	longest: usize
) -> Result<(), io::Error> {
	for child in node.children(&mut node.walk()) {
		write!(
			file,
			"{}",
			" ".repeat(longest.saturating_sub(child.kind().len()))
		)?;
		run_node(&child, source, file, " ")?;
		write!(file, "\n")?;
	}

	Ok(())
}

fn run_clause(node: &Node, source: &str, file: &mut File) -> Result<(), io::Error> { todo!() }

fn run_node(node: &Node, source: &str, file: &mut File, append: &str) -> Result<(), io::Error> {
	for child in node.children(&mut node.walk()) {
		if child.child_count() == 0 {
			// BUG: select if <Space>, <CR> or nothing
			write!(file, "{} ", child.utf8_text(source.as_bytes()).unwrap())?;
		} else if child.kind().ends_with("statement") {
			// TODO: run_substatement()
		} else if child.kind() == "dotted_name" {
			run_dotted_name(&child, source, file)?;
		} else if child.kind() == "binary_expression" {
			run_binary_expression(&child, source, file)?;
		} else if child.kind().ends_with("subexpression") {
			run_node(&child, source, file, " ")?;
			todo!()
		} else {
			run_node(&child, source, file, " ")?;
		}
	}

	Ok(())
}

fn run_binary_expression(node: &Node, source: &str, file: &mut File) -> Result<(), io::Error> {
	todo!("second node has spaces around");
}

fn run_dotted_name(node: &Node, source: &str, file: &mut File) -> Result<(), io::Error> {
	for child in node.children(&mut node.walk()) {
		if child.child_count() == 0 {
			// TODO: call run_node with "" as next_character
			write!(file, "{}", child.utf8_text(source.as_bytes()).unwrap())?;
		} else {
			run_dotted_name(&child, source, file)?;
		}
	}

	Ok(())
}

// fn run_nodes(
// 	node: &Node,
// 	source: &str,
// 	file: &mut File,
// 	padding: &str,
// 	ending: &str
// ) -> Result<()> {
// 	for child in node.children(&mut node.walk()) {
// 		if child.kind().ends_with("statement") {
// 			let _longest = child
// 				.children(&mut node.walk())
// 				.map(|child| child.kind().len())
// 				.max()
// 				.unwrap_or(0);
//
// 			run_nodes(&child, &source, file, padding, " ")?;
//
// 			// TODO: only if there are other statements in the same level
// 			write!(file, ";\n")?;
// 		} else if child.kind().ends_with("clause") {
// 			run_nodes(&child, &source, file, padding, " ")?;
//
// 			write!(file, "\n")?;
// 		} else if child.kind().ends_with("subexpression") {
// 			run_nodes(&child, &source, file, padding, " ")?;
// 		} else if child.kind() == "dotted_name" {
// 			run_nodes(&child, &source, file, padding, "")?;
// 		} else if child.kind() == ";" {
// 			run_nodes(&child, &source, file, padding, "")?;
// 		} else if child.child_count() == 0 {
// 			write!(
// 				file,
// 				"{}{}{}",
// 				padding,
// 				child.utf8_text(source.as_bytes()).unwrap(),
// 				ending
// 			)?;
//
// 			run_nodes(&child, &source, file, padding, " ")?;
// 		} else {
// 			run_nodes(&child, &source, file, padding, " ")?;
// 		}
// 	}
//
// 	Ok(())
// }

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
