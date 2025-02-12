use crate::db::connection::*;
use crate::db::models::*;
use crate::db::schema::busmonitormanager::dsl::*;
use crate::models::AppState;
use diesel::prelude::*;
use once_cell::sync::Lazy;

// 全局连接池实例
pub static MONITORS: Lazy<Vec<BusMonitorManager>> = Lazy::new(|| {
    let m = Monitor {};
    let monitors = m.query_monitors().unwrap();
    monitors
});

pub struct Monitor {}

impl Monitor {
    fn query_monitors(&self) -> std::io::Result<Vec<BusMonitorManager>> {
        let conn = &mut POOL
            .get()
            .map_err(|e| {
                eprintln!("Failed to get database connection: {}", e);
            })
            .unwrap();

        match busmonitormanager
            .select(BusMonitorManager::as_select())
            .load::<BusMonitorManager>(conn)
        {
            Ok(response) => Ok(response),
            Err(e) => Ok(Vec::new()),
        }
    }
}
