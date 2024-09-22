use super::process::ProcessNode;
use std::collections::HashMap;

pub fn print_tree(
    tree: &HashMap<i32, ProcessNode>,
    pid: i32,
    depth: i32,
    prefix: &str,
    is_last: bool,
) {
    if let Some(node) = tree.get(&pid) {
        // 打印进程信息
        if depth == 0 {
            println!("{}{}({})", prefix, node.name, node.pid);
        } else {
            println!("{}+-{}({})", prefix, node.name, node.pid);
        }

        if node.children.is_none() {
            return;
        }

        let new_prefix = if depth == 0 {
            format!("{}  ", prefix)
        } else {
            match is_last {
                true => format!("{}  ", prefix),
                false => format!("{}| ", prefix),
            }
        };
        // 递归打印子进程
        for (child_index, child_pid) in node.children.as_ref().unwrap().iter().enumerate() {
            print_tree(
                tree,
                *child_pid,
                depth + 1,
                &new_prefix,
                child_index == node.children.as_ref().unwrap().len() - 1,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_tree() {
        let mut tree = HashMap::new();
        let mut proc = ProcessNode::new();
        proc.pid = 1;
        proc.ppid = 0;
        proc.name = "systemd".to_string();
        proc.children = Some(vec![2]);
        tree.insert(1, proc);

        let mut proc = ProcessNode::new();
        proc.pid = 2;
        proc.name = "bash".to_string();
        proc.ppid = 1;
        proc.children = Some(vec![3, 4]);
        tree.insert(2, proc);

        let mut proc = ProcessNode::new();
        proc.pid = 3;
        proc.name = "ls".to_string();
        proc.ppid = 2;
        tree.insert(3, proc);

        let mut proc = ProcessNode::new();
        proc.pid = 4;
        proc.name = "cat".to_string();
        proc.ppid = 2;
        tree.insert(4, proc);

        print_tree(&tree, 1, 0, "", true);
    }
}
