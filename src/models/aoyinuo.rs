use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceStats {
    pub devtypeid: i64,
    pub online_count: usize,
    pub offline_count: usize,
}
