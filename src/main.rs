const SQUARE_GEN_DEPTH: u32 = 30;

#[derive(Clone, Debug)]
struct TreeNode {
    value: u32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn push(&mut self, node: TreeNode) -> u32 {
        self.children.push(node);
        return self.children.len() as u32 - 1;
    }
    fn new(num: u32) -> Self {
        return TreeNode {
            value: num,
            children: vec![],
        };
    }
}

#[derive(Debug)]
struct SqrtResult {
    whole: u32,
    frac:  u32,
    tree: TreeNode,
}

impl SqrtResult {
    fn new(whole: u32, frac: u32, tree: &TreeNode) -> Self {
        return SqrtResult {
            whole,
            frac,
            tree: tree.clone(),
        }
    }
}

struct Calc {
    squares: Vec<u32>,
    num: u32,
}

impl Calc {
    fn new(num: u32) -> Self {
        let mut calc = Calc {
            squares: vec![],
            num,
        };
        calc.gen_squares(SQUARE_GEN_DEPTH);
        return calc;
    }
    /// Generate square numbers up to the depth
    fn gen_squares(&mut self, depth: u32) {
        // Loop till depth, don't include 1
        for i in 2..(depth+1) {
            self.squares.push(i*2);
        }
    }
    /// Recursively generate simplest radical form square root
    fn sqrt(&self, result: Option<SqrtResult>) -> SqrtResult {
        let r = match result {
            Some(r) => r,
            _ => SqrtResult {
                whole: self.num,
                frac:  0,
                tree:  TreeNode::new(self.num),
            },
        };
        let mut node = r.tree;
        let num = r.whole;
        // Check if number is square, if so return root
        let decimal_root = (num as f32).sqrt();
        match decimal_root.fract() {
            0.0 => {
                node.children.push(TreeNode::new(decimal_root as u32));
                return SqrtResult{
                    whole: decimal_root as u32,
                    frac:  0,
                    tree: node,
                };
            },
            _   => {}
        };
        // Otherwise, find square number divisor
        for i in &self.squares {
            let node = &mut node;
            // If the square number goes in evenly
            if (num as f32 / *i as f32).fract() == 0.0 {
                // Create children nodes
                let _ = node.push(TreeNode::new(*i));
                let _ = node.push(TreeNode::new(num / i));
                // Recurse
                let whole  = self.sqrt(Some(SqrtResult::new(*i,0,node))).whole
                    * self.sqrt(Some(SqrtResult::new(num/i,0,node))).whole;
                let frac  = self.sqrt(Some(SqrtResult::new(*i,0,node))).frac
                    + self.sqrt(Some(SqrtResult::new(num/i,0,node))).frac;
                let result = SqrtResult::new(whole, frac, node);

                return result;
            }
        }
        // It's prime, return 1 * sqrt(num)
        return SqrtResult::new(1, num, &node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_num() {
        let calc = Calc::new(16);
        let root = calc.sqrt(None);
        assert_eq!(root.whole, 4);
    }
    #[test]
    fn test_prime() {
        let calc = Calc::new(17);
        let root = calc.sqrt(None);
        assert_eq!(root.frac, 17);
    }
    #[test]
    fn test_not_square_num() {
        let calc = Calc::new(12);
        let root = calc.sqrt(None);
        assert_eq!(root.whole, 2);
        assert_eq!(root.frac,  3);
    }
    #[test]
    fn test_more_steps() {
        let calc = Calc::new(48);
        let root = calc.sqrt(None);
        assert_eq!(root.whole, 4);
        assert_eq!(root.frac,  3);
    }
}
