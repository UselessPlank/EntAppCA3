/***************************************************************************************
*    Title: Huffman Tree CA2 - Algorithms & Computations
*    Author: ChatGPT & Jason O'Connor
*    Date: 28/04/2025
*    Code Version: 1.0
*    Type: Computer Program for Huffman Encoding and Decoding
*    Availability: https://github.com/UselessPlank/AlgoCompCA2.git
***************************************************************************************/

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};

#[derive(Debug, Clone)]
struct HuffmanNode {
    character: Option<char>,
    frequency: u32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

pub struct HuffmanTree {
    root: Option<Box<HuffmanNode>>,
    encoding_map: HashMap<char, String>,
    decoding_map: HashMap<String, char>,
}

impl HuffmanTree {
    pub fn new() -> Self {
        HuffmanTree {
            root: None,
            encoding_map: HashMap::new(),
            decoding_map: HashMap::new(),
        }
    }

    pub fn build_from_file(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        self.build_from_text(&contents);
        Ok(())
    }

    pub fn build_from_text(&mut self, text: &str) {
        let mut frequency_map = HashMap::new();
        for c in text.chars() {
            *frequency_map.entry(c).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();
        for (character, frequency) in frequency_map {
            heap.push(HuffmanNode {
                character: Some(character),
                frequency,
                left: None,
                right: None,
            });
        }

        // Handle the case where there's only one unique character
        if heap.len() == 1 {
            let node = heap.pop().unwrap();
            self.root = Some(Box::new(node));
            self.build_encoding_maps();
            return;
        }

        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            let parent = HuffmanNode {
                character: None,
                frequency: left.frequency + right.frequency,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };
            heap.push(parent);
        }

        if let Some(root) = heap.pop() {
            self.root = Some(Box::new(root));
            self.build_encoding_maps();
        }
    }

    fn build_encoding_maps(&mut self) {
        self.encoding_map.clear();
        self.decoding_map.clear();
        
        if let Some(root) = &self.root {
            // Handle the case where there's only one character
            if let Some(character) = root.character {
                self.encoding_map.insert(character, "0".to_string());
                self.decoding_map.insert("0".to_string(), character);
                return;
            }

            let mut temp_map = HashMap::new();
            Self::build_encoding_maps_recursive(root, String::new(), &mut temp_map);
            
            for (character, code) in temp_map {
                self.encoding_map.insert(character, code.clone());
                self.decoding_map.insert(code, character);
            }
        }
    }

    fn build_encoding_maps_recursive(node: &HuffmanNode, code: String, map: &mut HashMap<char, String>) {
        if let Some(character) = node.character {
            map.insert(character, code);
        } else {
            if let Some(left) = &node.left {
                Self::build_encoding_maps_recursive(left, code.clone() + "0", map);
            }
            if let Some(right) = &node.right {
                Self::build_encoding_maps_recursive(right, code + "1", map);
            }
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len() * 2);
        for c in text.chars() {
            if let Some(code) = self.encoding_map.get(&c) {
                result.push_str(code);
            }
        }
        result
    }

    pub fn decode(&self, encoded: &str) -> String {
        if self.decoding_map.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(encoded.len() / 2);
        let mut current_code = String::with_capacity(32);
        
        for bit in encoded.chars() {
            if bit != '0' && bit != '1' {
                continue;
            }
            current_code.push(bit);
            
            if let Some(&character) = self.decoding_map.get(&current_code) {
                result.push(character);
                current_code.clear();
            }
        }
        
        result
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        
        for (character, code) in &self.encoding_map {
            let escaped_char = match character {
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                '\\' => "\\\\".to_string(),
                ':' => "\\:".to_string(),
                _ => character.to_string(),
            };
            writeln!(writer, "{}:{}", escaped_char, code)?;
        }
        
        Ok(())
    }

    pub fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        
        self.encoding_map.clear();
        self.decoding_map.clear();
        
        for line in io::BufRead::lines(reader) {
            let line = line?;
            if let Some((character, code)) = line.split_once(':') {
                let c = match character {
                    "\\n" => '\n',
                    "\\r" => '\r',
                    "\\t" => '\t',
                    "\\:" => ':',
                    "\\\\" => '\\',
                    s if s.len() == 1 => s.chars().next().unwrap(),
                    _ => continue,
                };
                self.encoding_map.insert(c, code.to_string());
                self.decoding_map.insert(code.to_string(), c);
            }
        }
        
        Ok(())
    }
}
