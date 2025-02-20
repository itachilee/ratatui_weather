use crate::db::connection::*;
use crate::db::models::*;
use crate::db::schema::busdevtypemanager::dsl::*;
use crate::db::schema::busmonitormanager::dsl::*;
use crate::db::schema::system_security_info::dsl::*;
use crate::models::PaginatedResult;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
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

        diesel::insert_into(system_security_info)
            .values(&new_info)
            .execute(conn)
            .expect("Error inserting system security info");
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
