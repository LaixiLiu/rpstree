// 表示进程状态的枚举
// Running: 运行中
// Sleeping: 睡眠中
// Zombie: 僵尸进程
// TracingStop: 被跟踪停止
// Dead: 死亡
// Idle: 空闲
#[derive(Debug, Clone)]
pub enum ProcessState {
    Running,
    Sleeping,
    Zombie,
    TracingStop,
    Dead,
    Idle,
}

// 表示进程的结构体
// pid: 进程ID
// ppid: 父进程ID
// name: 进程名
// state: 进程状态
#[derive(Debug, Clone)]
pub struct ProcessNode {
    pub pid: i32,
    pub ppid: i32,
    pub name: String,
    pub state: ProcessState,
    pub children: Option<Vec<i32>>,
}

impl ProcessNode {
    pub fn new() -> ProcessNode {
        ProcessNode {
            pid: 1,
            ppid: 0,
            name: String::new(),
            state: ProcessState::Idle,
            children: None,
        }
    }

    pub fn add_child(&mut self, child: i32) {
        if self.children.is_none() {
            self.children = Some(Vec::new());
        }
        self.children.as_mut().unwrap().push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proc_add_child() {
        let mut proc = ProcessNode::new();
        proc.add_child(2);
        proc.add_child(3);
        assert_eq!(proc.children.unwrap(), vec![2, 3]);
    }
}