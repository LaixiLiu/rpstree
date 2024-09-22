use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Config {
    Normal,
    Help,
    Version,
    ProcessTree {
        numeric_sort: bool,
        pid_visiblity: bool,
    },
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        // 跳过第一个参数
        args.next();
        // 定义选项
        let mut numeric_sort = false;
        let mut pid_visiblity = false;
        let mut help = false;
        let mut version = false;
        // 解析选项
        for arg in args {
            match arg.as_str() {
                "-p" | "--pid" => pid_visiblity = true,
                "-n" | "--numeric-sort" => numeric_sort = true,
                "-h" | "--help" => help = true,
                "-v" | "--version" => version = true,
                _ => {
                    return Err(Box::from(format!("Unknown option: {}", arg)));
                }
            }
        }
        // 检查选项
        // help与version只能单独使用
        if help && (version || numeric_sort || pid_visiblity) {
            return Err(Box::from("Help option can't be used with other options"));
        }
        if version && (help || numeric_sort || pid_visiblity) {
            return Err(Box::from("Version option can't be used with other options"));
        }
        // 返回配置
        if help {
            Ok(Config::Help)
        } else if version {
            Ok(Config::Version)
        } else if numeric_sort || pid_visiblity {
            Ok(Config::ProcessTree {
                numeric_sort,
                pid_visiblity,
            })
        } else {
            Ok(Config::Normal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build_p_n() {
        let args = vec!["rpstree".to_string(), "-p".to_string(), "-n".to_string()];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(
            config,
            Config::ProcessTree {
                numeric_sort: true,
                pid_visiblity: true
            }
        );
        let args = vec![
            "rpstree".to_string(),
            "--pid".to_string(),
            "--numeric-sort".to_string(),
        ];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(
            config,
            Config::ProcessTree {
                numeric_sort: true,
                pid_visiblity: true
            }
        );
    }
    #[test]
    fn test_config_build_h() {
        let args = vec!["rpstree".to_string(), "-h".to_string()];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config, Config::Help);
        let args = vec!["rpstree".to_string(), "--help".to_string()];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config, Config::Help);
    }

    #[test]
    fn test_config_build_v() {
        let args = vec!["rpstree".to_string(), "-v".to_string()];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config, Config::Version);
        let args = vec!["rpstree".to_string(), "--version".to_string()];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config, Config::Version);
    }

    #[test]
    #[should_panic]
    fn test_config_build_h_fail() {
        let args = vec!["rspstree".to_string(), "-h".to_string(), "-p".to_string()];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config, Config::Version);
    }

    #[test]
    #[should_panic]
    fn test_config_build_v_fail() {
        let args = vec!["rspstree".to_string(), "-v".to_string(), "-p".to_string()];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config, Config::Version);
    }
}
