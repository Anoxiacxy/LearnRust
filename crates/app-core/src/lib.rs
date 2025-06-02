pub mod cli;
pub mod monitor;
pub mod services;
pub mod utils;

// 重新导出常用的类型
pub use cli::{Cli, Commands};
pub use monitor::MarketMonitor;
pub use services::{User, UserService, UserServiceImpl};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
