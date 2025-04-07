use core::fmt;
use std::{collections::VecDeque, io::{self, Write}, ops::{Index, IndexMut}};

use clap::Parser;

const SQUARE_GEN_DEPTH: u32 = 30;
const RESET_COLOR: &str = "\x1b[0m";
const HIGHL_COLOR: &str = "\x1b[34m";

#[derive(Clone, Debug)]
struct TreeNode {
    value: u32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(num: u32) -> Self {
        return TreeNode {
            value: num,
            children: vec![],
        };
    }
    fn print(&self) {
        let mut queue: VecDeque<TreeNode> = vec![].into();
        queue.push_back(self.clone());
        while queue.len() > 0 {
            for i in queue.clone() {
                if i.value != 0 {
                }
                // Detect dummy character
                if i.value == 0 {
                    println!("");
                    queue.pop_front();
                    continue;
                } else {
                    // Print real character
                    print!("{} ",i.value);
                    queue.pop_front();
                }
                for j in i.children {
                    queue.push_back(j);
                }
                // Insert dummy newline character
                if queue.len() > 0 {
                    queue.push_front(TreeNode::new(0));
                }
            }
        }
    }
}


impl Index<usize> for TreeNode {
    type Output = TreeNode;
    fn index(&self, index: usize) -> &Self::Output {
        let mut pos: &TreeNode = self;
        for i in format!("{:b}",index).chars() {
            pos = &pos.children[i.to_digit(10).expect("Error converting index") as usize];
        }
        return pos;
    }
}
impl IndexMut<usize> for TreeNode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut pos: &mut TreeNode = self;
        for i in format!("{:b}",index).chars() {
            pos = &mut pos.children[i.to_digit(10).expect("Error converting index") as usize];
        }
        return pos;
    }
}

#[derive(Debug)]
struct SqrtResult {
    whole: u32,
    frac:  u32,
    tree: TreeNode,
    tree_pos: u32,
}

impl SqrtResult {
    fn new(whole: u32, frac: u32, tree: TreeNode, tree_pos: u32) -> Self {
        return SqrtResult {
            whole,
            frac,
            tree: tree.clone(),
            tree_pos,
        }
    }
}

// Implement display so the result can be printed by println! in a standard way
impl fmt::Display for SqrtResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}√{}{}",self.whole,HIGHL_COLOR,self.frac,RESET_COLOR)
    }
}

struct Calc {
    squares: Vec<u32>,
    num: u32,
    tree: TreeNode,
}

impl Calc {
    fn new(num: u32) -> Self {
        let mut calc = Calc {
            squares: vec![],
            num,
            tree: TreeNode::new(num),
        };
        calc.gen_squares(SQUARE_GEN_DEPTH);
        return calc;
    }
    /// Generate square numbers up to the depth
    fn gen_squares(&mut self, depth: u32) {
        // Loop till depth, don't include 1
        for i in 2..(depth+1) {
            self.squares.push(i*i);
        }
    }
    // Find square root, no recurse. Return value: (left, right, do_recurse)
    fn find_sqrt(&self, num: u32) -> (u32, u32, bool) {
        // Check if number is square, if so return root
        let decimal_root = (num as f32).sqrt();
        match decimal_root.fract() {
            0.0 => return (decimal_root as u32, 0, false),
            _   => {}
        }

        // Look through squares, check divisible
        for i in &self.squares {
            // Check if divisible
            match (num as f32 / *i as f32).fract() {
                0.0 => return (*i, num/i, true),
                _ => {},
            }
        }

        // Prime, return (1, num)
        return (1, num, false);
    }
    /// Recursively generate simplest radical form square root
    fn sqrt(&mut self, result: Option<SqrtResult>) -> SqrtResult {
        let r = match result {
            Some(r) => r,
            _ => SqrtResult {
                whole: self.num,
                frac:  0,
                tree:  TreeNode::new(self.num),
                tree_pos: 0b0,
            },
        };
        let num = r.whole;

        let root = self.find_sqrt(num);

        if self.tree.children.len() > 0 {
            self.tree[r.tree_pos as usize].children.push(TreeNode::new(root.0));
            self.tree[r.tree_pos as usize].children.push(TreeNode::new(root.1));
        } else {
            self.tree.children.push(TreeNode::new(root.0));
            self.tree.children.push(TreeNode::new(root.1));
        }

        if root.2 {
            let l_tree = r.tree_pos << 1;
            let r_tree = (r.tree_pos << 1) | 0b1;
            let a = self.sqrt(
                Some(
                    SqrtResult::new(
                        root.0,
                        0,
                        self.tree.clone(),
                        l_tree,
                    )
                )
            );
            let b = self.sqrt(
                Some(
                    SqrtResult::new(
                        root.1,
                        0,
                        self.tree.clone(),
                        r_tree,
                    )
                )
            );
            return SqrtResult::new(a.whole * b.whole, a.frac + b.frac, r.tree.clone(), r.tree_pos);
        }

        return SqrtResult::new(root.0, root.1, self.tree.clone(), r.tree_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_num() {
        let mut calc = Calc::new(16);
        let root = calc.sqrt(None);
        assert_eq!(root.whole, 4);
    }
    #[test]
    fn test_prime() {
        let mut calc = Calc::new(17);
        let root = calc.sqrt(None);
        assert_eq!(root.frac, 17);
    }
    #[test]
    fn test_not_square_num() {
        let mut calc = Calc::new(12);
        let root = calc.sqrt(None);
        assert_eq!(root.whole, 2);
        assert_eq!(root.frac,  3);
    }
    #[test]
    fn test_more_steps() {
        let mut calc = Calc::new(48);
        let root = calc.sqrt(None);
        assert_eq!(root.whole, 4);
        assert_eq!(root.frac,  3);
    }
    #[test]
    fn test_not_square_num_2() {
        let mut calc = Calc::new(24);
        let root = calc.sqrt(None);
        assert_eq!(root.whole, 2);
        assert_eq!(root.frac,  6);
    }
}

/// Find the square roots of numbers in simplest radical form
#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional: number to find square root of (if not passed, will enter REPL)
    number: Option<u32>,
    /// If passed, tree will not be printed
    #[clap(long,short='t',action)]
    notree: bool,
}

fn main() {
    let args = Args::parse();
    match args.number {
        Some(n) => {
            // Print root
            let mut calc = Calc::new(n);
            let root = calc.sqrt(None);
            println!("{root}");
            // Print Tree
            if !args.notree {
                println!("=============");
                calc.tree.print();
            }
            return;
        },
        None => {}
    }

    // Enter REPL
    loop {
        // Print prompt
        print!("Get root of number: ");
        io::stdout()
            .flush()
            .unwrap();
        // Read command line input
        let input = &mut String::new();
        let _ = io::stdin()
            .read_line(input)
            .expect("Invalid input");
        // Convert input to number
        let input_num = input
            .trim()
            .parse()
            .unwrap();
        // Get root and print it
        let mut calc = Calc::new(input_num);
        let root = calc.sqrt(None);
        println!("{root}");
        // Print Tree
        if !args.notree {
            println!("=============");
            root.tree.print();
        }
    }
}
