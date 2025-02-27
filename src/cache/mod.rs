use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 简单内存缓存结构体
struct SimpleCache {
    items: HashMap<String, (String, Instant)>,
    ttl: Duration,
}

impl SimpleCache {
    /// 创建一个新的缓存实例，设置默认过期时间
    fn new(ttl: Duration) -> Self {
        SimpleCache {
            items: HashMap::new(),
            ttl,
        }
    }

    /// 插入或更新缓存值
    fn put(&mut self, key: String, value: String) {
        let expires = Instant::now() + self.ttl;
        self.items.insert(key, (value, expires));
    }

    /// 获取缓存值（若未过期）
    fn get(&mut self, key: &str) -> Option<String> {
        if let Some((value, expires)) = self.items.get(key) {
            if Instant::now() < *expires {
                return Some(value.clone());
            }
        }
        None
    }

    /// 删除指定键的缓存
    fn remove(&mut self, key: &str) {
        self.items.remove(key);
    }

    /// 清空所有缓存
    fn clear(&mut self) {
        self.items.clear();
    }
}
