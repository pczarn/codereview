#[derive(Debug)]
struct TreeNode {
    ch: char,
    count: u32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(ch: char) -> Self {
        Self {
            ch,
            count: 0,
            children: vec![],
        }
    }

    fn get_value(&self) -> char { self.ch }

    fn increment_count(&mut self) { self.count += 1; }
}

fn build_trie(paths: &Vec<String>) -> TreeNode {
    let mut root = TreeNode::new('\0');
    for path in paths {
        // start at the root node
        let mut current_node = &mut root;
        for ch in path.as_bytes() {
            let ch = *ch as char;
            println!("Current char: {}", ch);
            // for each child of the current node, check if the current character matches
            let maybe_found = current_node.children.iter_mut().position(|child|
                child.get_value() == ch
            );
            match maybe_found {
                Some(index) => {
                    println!("Found matching char: {}", ch);
                    current_node.children[index].increment_count();
                    current_node = &mut current_node.children[index];
                }
                None => {
                    let new_node = TreeNode::new(ch);
                    current_node.children.push(new_node);
                    current_node = current_node.children.last_mut().unwrap();
                }
            }
        }
    }
    root
}

fn main() {
    let paths = vec!["abc".to_string(), "abd".to_string()];
    let trie = build_trie(&paths);
    println!("{:?}", trie);
}