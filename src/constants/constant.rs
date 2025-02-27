use std::collections::HashMap;

use crate::db::connection::*;
use crate::db::models::*;
use crate::db::schema::busdevtypemanager::dsl::busdevtypemanager;
use crate::db::schema::busmonitormanager::devtypeid;
use crate::db::schema::busmonitormanager::dsl::busmonitormanager;
use crate::db::schema::busmonitormanager::onlinestate;
use crate::db::schema::system_security_info::dsl::system_security_info;
use crate::db::schema::warnings::dsl::*;
use crate::modbus::monitor_threshold::WarningInfo;
use crate::models::DeviceStats;
use crate::models::PaginatedResult;
use chrono::DateTime;
use chrono::Utc;
use diesel::dsl::count;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use log::info;
use once_cell::sync::Lazy;

// 全局连接池实例
pub static MONITORS: Lazy<Vec<BusMonitorManager>> = Lazy::new(|| {
    let m = Monitor {};
    let monitors = m.query_monitors().unwrap();
    monitors
});

pub static DEVTYPES: Lazy<Vec<BusDevTypeManager>> = Lazy::new(|| {
    let m = Monitor {};
    let res: Vec<BusDevTypeManager> = m.query_dev_types().unwrap();
    res
});

pub struct Monitor;

impl Monitor {
    pub fn insert_security_info(&self) {
        let conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        > = &mut POOL
            .get()
            .map_err(|e| {
                eprintln!("Failed to get database connection: {}", e);
            })
            .unwrap();

        // 获取当前带有时区信息的时间
        let now: DateTime<Utc> = Utc::now();
        // 将其转换为 NaiveDateTime
        let t = now.naive_utc();
        let new_info = NewSystemSecurityInfo {
            start_date: t,
            end_date: None,
        };

        match system_security_info.first::<SystemSecurityInfo>(conn) {
            Ok(res) => {}
            Err(NotFound) => {
                match diesel::insert_into(system_security_info)
                    .values(&new_info)
                    .execute(conn)
                {
                    Ok(res) => {
                        println!("insert system security info success");
                    }
                    Err(e) => {
                        println!("Error insert system security info {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error query system security info {}", e);
            }
        }
    }

    pub fn insert_warning(&self, warning: WarningInfo) {
        // let desc = format!(
        //     "{} | 氧气传感器数据触发预警！预警信息: {:?}",
        //     chrono::Local::now(),
        //     warning
        // );

        let desc = warning.to_string();
        // 准备要插入的数据
        let warning_info = WarningInsert {
            sensor_type: &warning.sensor_type.to_string(),
            dev_ip: &warning.dev_ip,
            value: warning.value,
            threshold: warning.threshold,
            reason: 1,
            description: &desc,
            timestamp: Utc::now().naive_local(), // 注意使用 UTC 时间
        };

        let conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        > = &mut POOL
            .get()
            .map_err(|e| {
                eprintln!("Failed to get database connection: {}", e);
            })
            .unwrap();

        match diesel::insert_into(warnings)
            .values(&warning_info)
            .execute(conn)
        {
            Ok(res) => {
                info!("insert warning info success");
            }
            Err(e) => {
                info!("Error insert warning info {}", e);
            }
        }
    }

    pub fn query_security_info(&self) -> SystemSecurityInfo {
        let conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        > = &mut POOL
            .get()
            .map_err(|e| {
                eprintln!("Failed to get database connection: {}", e);
            })
            .unwrap();
        let i = system_security_info
            .first::<SystemSecurityInfo>(conn)
            .unwrap();
        i
    }

    pub fn query_warning(&self) -> WarningQuery {
        let conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        > = &mut POOL
            .get()
            .map_err(|e| {
                eprintln!("Failed to get database connection: {}", e);
            })
            .unwrap();
        let i = warnings
            .order_by(timestamp.desc())
            .first::<WarningQuery>(conn)
            .unwrap();
        i
    }
    fn query_monitors(&self) -> std::io::Result<Vec<BusMonitorManager>> {
        let conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        > = &mut POOL
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

    fn query_dev_types(&self) -> std::io::Result<Vec<BusDevTypeManager>> {
        let conn = &mut POOL
            .get()
            .map_err(|e| {
                eprintln!("Failed to get database connection: {}", e);
            })
            .unwrap();

        match busdevtypemanager
            .select(BusDevTypeManager::as_select())
            .load::<BusDevTypeManager>(conn)
        {
            Ok(response) => Ok(response),
            Err(e) => Ok(Vec::new()),
        }
    }

    fn query_ipphones(&self) -> Vec<BusMonitorManager> {
        MONITORS
            .clone()
            .iter()
            .cloned()
            .filter(|a| a.devtypeid == 39)
            // .map()
            .collect::<Vec<BusMonitorManager>>()
    }

    fn query_cameras(&self) -> Vec<BusMonitorManager> {
        MONITORS
            .clone()
            .iter()
            .cloned()
            .filter(|a| a.devtypeid == 33)
            // .map()
            .collect::<Vec<BusMonitorManager>>()
    }

    pub fn query_ipphones_by_pages(
        &self,
        page: i64,
        page_size: i64,
    ) -> PaginatedResult<BusMonitorManager> {
        paginate(&self.query_ipphones(), page, page_size)
    }

    pub fn query_cameras_by_pages(
        &self,
        page: i64,
        page_size: i64,
    ) -> PaginatedResult<BusMonitorManager> {
        paginate(&self.query_cameras(), page, page_size)
    }

    pub fn count_online_devices(&self) -> Vec<DeviceStats> {
        let mut groups: HashMap<i64, (usize, usize)> = HashMap::new();

        for device in MONITORS.clone() {
            let dev_type = device.devtypeid;
            if device.onlinestate {
                groups.entry(dev_type).or_insert((0, 0)).0 += 1;
            } else {
                groups.entry(dev_type).or_insert((0, 0)).1 += 1;
            }
        }

        // 将 HashMap 转换为 Vec
        let res: Vec<DeviceStats> = groups
            .into_iter()
            .map(|(dev_type, (online, offline))| DeviceStats {
                devtypeid: dev_type,
                online_count: online,
                offline_count: offline,
            })
            .collect();
        res
    }
}

pub fn paginate<T>(data: &Vec<T>, page: i64, page_size: i64) -> PaginatedResult<T>
where
    T: Clone,
{
    let total = data.len() as i64;
    let total_pages = (total + page_size - 1) / page_size;

    let start_index = ((page - 1) * page_size) as usize;
    let end_index = (start_index + page_size as usize).min(data.len());

    let items = if start_index < data.len() {
        data[start_index..end_index].to_vec()
    } else {
        vec![]
    };

    let has_prev_page = page > 1;
    let has_next_page = page < total_pages;

    PaginatedResult {
        page,
        page_size,
        total,
        total_pages,
        items,
        has_prev_page,
        has_next_page,
    }
}
