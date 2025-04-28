/***************************************************************************************
*    Title: Huffman Tree CA2 - Algorithms & Computations
*    Author: ChatGPT & Jason O'Connor
*    Date: 28/04/2025
*    Code Version: 1.0
*    Type: Computer Program for Huffman Encoding and Decoding
*    Availability: https://github.com/UselessPlank/AlgoCompCA2.git
***************************************************************************************/

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::PathBuf;

mod huffman_tree;
use huffman_tree::HuffmanTree;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        tree: PathBuf,
    },
    Decode {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        tree: PathBuf,
    },
}

fn read_file_contents(path: &PathBuf) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file_contents(path: &PathBuf, contents: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { input, output, tree } => {
            let mut huffman = HuffmanTree::new();
            let contents = read_file_contents(&input)?;
            
            huffman.build_from_text(&contents);
            let encoded = huffman.encode(&contents);
            write_file_contents(&output, &encoded)?;
            
            huffman.save_to_file(tree.to_str().unwrap())?;
        }
        Commands::Decode { input, output, tree } => {
            let mut huffman = HuffmanTree::new();
            huffman.load_from_file(tree.to_str().unwrap())?;
            
            let contents = read_file_contents(&input)?;
            let decoded = huffman.decode(&contents);
            write_file_contents(&output, &decoded)?;
        }
    }

    Ok(())
}
