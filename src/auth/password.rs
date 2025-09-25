// 导入bcrypt密码哈希库的相关功能
use bcrypt::{DEFAULT_COST, hash, verify};
// 导入错误类型
use std::error::Error;

// 密码哈希函数
pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    // 使用bcrypt对密码进行哈希，使用默认成本系数
    Ok(hash(password, DEFAULT_COST)?)
}

// 密码验证函数
pub fn verify_password(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
    // 验证密码是否与哈希值匹配
    Ok(verify(password, hash)?)
}

// 测试模块
#[cfg(test)]
mod tests {
    // 导入父模块的所有内容
    use super::*;

    // 测试密码哈希功能
    #[test]
    fn test_password_hashing() -> Result<(), Box<dyn Error>> {
        let password = "test_password";
        // 对密码进行哈希
        let hash = hash_password(password)?;
        // 验证正确密码应该通过
        assert!(verify_password(password, &hash)?);
        // 验证错误密码应该失败
        assert!(!verify_password("wrong_password", &hash)?);
        Ok(())
    }
}
