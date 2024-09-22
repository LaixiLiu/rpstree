pub mod cli;
mod output;
mod process;

use cli::Config;
use process::{ProcessNode, ProcessState};
use std::{collections::HashMap, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let process_list = read_process()?;
    let mut process_tree = build_process_tree(process_list)?;
    match config {
        Config::Help => output::print_help_info(),
        Config::Version => output::print_version(),
        Config::ProcessTree {
            numeric_sort,
            pid_visiblity,
        } => {
            if numeric_sort {
                // 将子进程按照pid升序排列
                for (_, node) in process_tree.iter_mut() {
                    node.children.as_mut().map(|c| c.sort());
                }
            }
            if pid_visiblity {
                output::print_tree_with_pid(&process_tree, 1, 0, "", true);
            } else {
                output::print_tree_without_pid(&process_tree, 1, 0, "", true);
            }
        }
        Config::Normal => output::print_tree_without_pid(&process_tree, 1, 0, "", true),
    }
    Ok(())
}

// 构建进程树
fn build_process_tree(
    process_list: Vec<ProcessNode>,
) -> Result<HashMap<i32, ProcessNode>, Box<dyn Error>> {
    let mut tree = HashMap::new();

    // 初始化所有节点
    for proc in &process_list {
        tree.insert(proc.pid, proc.clone());
    }

    // 构建父子关系
    for proc in &process_list {
        if proc.ppid != 0 {
            tree.get_mut(&proc.ppid)
                .ok_or("Failed to get parent process")?
                .add_child(proc.pid);
        }
    }
    Ok(tree)
}

// 生成进程列表
fn read_process() -> Result<Vec<ProcessNode>, Box<dyn Error>> {
    // hashmap保存进程ID和进程信息的映射
    let mut process_list: Vec<ProcessNode> = Vec::new();

    // 读取/proc目录下的所有目录
    let process_dir = fs::read_dir("/proc")?.filter(|t| {
        let t = t.as_ref().unwrap();
        t.file_type().unwrap().is_dir()
            && t.file_name()
                .to_str()
                .unwrap()
                .parse::<i32>()
                .ok()
                .is_some()
    });
    // 遍历所有进程目录，解析进程信息
    for entry in process_dir {
        let proc = parse_process(
            entry?
                .file_name()
                .to_str()
                .ok_or("Missing process PID")?
                .parse()?,
        )?;
        process_list.push(proc);
    }

    Ok(process_list)
}

fn parse_process(pid: i32) -> Result<ProcessNode, Box<dyn Error>> {
    let mut proc = ProcessNode::new();
    proc.pid = pid;
    // 读取/proc/[pid]/status文件
    let status_file = fs::read_to_string(format!("/proc/{pid}/status")).unwrap();
    for line in status_file.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("Name:") => proc.name = parts.next().unwrap().to_string(),
            Some("PPid:") => {
                proc.ppid = parts.next().unwrap().parse().unwrap();
                break;
            }
            Some("State:") => {
                proc.state = match parts.next().unwrap() {
                    "R" => ProcessState::Running,
                    "S" => ProcessState::Sleeping,
                    "Z" => ProcessState::Zombie,
                    "T" => ProcessState::TracingStop,
                    "X" => ProcessState::Dead,
                    _ => ProcessState::Idle,
                };
            }
            _ => {}
        }
    }
    Ok(proc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_process_ok() {
        let pid = 1;
        let proc = parse_process(pid).unwrap();
        println!("{:?}", proc);
        assert_eq!(proc.pid, 1);
        assert_eq!(proc.ppid, 0);
    }

    #[test]
    #[should_panic]
    fn test_parse_process_err() {
        parse_process(114514).unwrap();
    }

    #[test]
    fn test_read_process() {
        let process_map = read_process().unwrap();
        println!("{:?}", process_map);
        assert!(process_map.len() > 0);
    }

    #[test]
    fn test_build_process_tree() {
        let process_list = read_process().unwrap();
        let process_tree = build_process_tree(process_list).unwrap();
        println!("{:?}", process_tree);
        assert!(process_tree.len() > 0);
    }
}
