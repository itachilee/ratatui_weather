use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::constants::constant::MONITORS;
use crate::db::connection::*;
use crate::db::models::*;
use crate::db::schema::busdevtypemanager::dsl::*;
use crate::db::schema::busmonitormanager::dsl::*;
use crate::models::PaginatedResult;
use diesel::prelude::*;

pub struct Aoyinuo;

impl Aoyinuo {
    fn query_monitors(&self) -> std::io::Result<Vec<BusMonitorManager>> {
        let conn: &mut diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>> = &mut POOL
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

    fn query_ipphones(&self) -> std::io::Result<Vec<BusMonitorManager>> {
        let conn = &mut POOL
            .get()
            .map_err(|e| {
                eprintln!("Failed to get database connection: {}", e);
            })
            .unwrap();

        match busmonitormanager
            .filter(devtypeid.eq(39))
            .select(BusMonitorManager::as_select())
            .load::<BusMonitorManager>(conn)
        {
            Ok(response) => Ok(response),
            Err(e) => Ok(Vec::new()),
        }
    }

    fn query_cameras(&self) -> std::io::Result<Vec<BusMonitorManager>> {
        let conn = &mut POOL
            .get()
            .map_err(|e| {
                eprintln!("Failed to get database connection: {}", e);
            })
            .unwrap();

        match busmonitormanager
            .filter(devtypeid.eq(33))
            .select(BusMonitorManager::as_select())
            .load::<BusMonitorManager>(conn)
        {
            Ok(response) => Ok(response),
            Err(e) => Ok(Vec::new()),
        }
    }

    pub fn get_posts(
        conn: &mut PgConnection,
        page: i64,
        per_page: i64,
    ) -> QueryResult<Vec<BusMonitorManager>> {
        let offset = (page - 1) * per_page;

        busmonitormanager
            .filter(devtypeid.eq(33))
            .limit(per_page)
            .offset(offset)
            .select(BusMonitorManager::as_select())
            .load::<BusMonitorManager>(conn)
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
