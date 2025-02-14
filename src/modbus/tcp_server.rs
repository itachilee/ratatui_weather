use crate::db::models::BusDevTypeManager;
use crate::modbus::constant::{DEVTYPES, MONITORS};
use crate::modbus::monitor_parser::{
    bytes_to_hex, HumiditySensorParser, OxygenSensorParser, SensorData, SensorParser,
    TemperatureSensorParser, WindSpeedSensorParser,
};
use crate::modbus::monitor_threshold::{handle_warning, OxygenThreshold};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use super::monitor_threshold::SensorType;

// 设备状态跟踪器
#[derive(Debug, Clone)]
struct DeviceStatusTracker {
    connected_devices: HashMap<SocketAddr, bool>,
    ip_whitelist: Vec<IpAddr>,
}

impl DeviceStatusTracker {
    fn new(whitelist: Vec<IpAddr>) -> Self {
        DeviceStatusTracker {
            connected_devices: HashMap::new(),
            ip_whitelist: whitelist,
        }
    }

    // 检查 IP 是否在白名单内
    fn is_ip_allowed(&self, addr: SocketAddr) -> bool {
        self.ip_whitelist.contains(&addr.ip())
    }

    // 记录设备连接
    fn device_connected(&mut self, addr: SocketAddr) {
        let dev_type_name = self.get_dev_type_name(&addr);
        if let Some(status) = self.connected_devices.get_mut(&addr) {
            if !*status {
                println!("{} Device {} connected", dev_type_name, addr);
                *status = true;
            }
        } else {
            println!("{} Device {} connected", dev_type_name, addr);
            self.connected_devices.insert(addr, true);
        }
    }

    // 记录设备断开连接
    fn device_disconnected(&mut self, addr: SocketAddr) {
        let dev_type_name = self.get_dev_type_name(&addr);
        if let Some(status) = self.connected_devices.get_mut(&addr) {
            if *status {
                println!("{} Device {} disconnected", dev_type_name, addr);
                *status = false;
            }
        }
    }
    fn get_dev_type_name(&self, addr: &SocketAddr) -> &'static str {
        let mut dev_type_name = "unknown";
        if let Some(monitor) = MONITORS
            .iter()
            .filter(|x| is_ip_legal(&x.devip))
            .find(|a| a.devip.trim().parse::<IpAddr>().unwrap() == addr.ip())
        {
            if let Some(dev_type) = DEVTYPES.iter().find(|a| a.id == monitor.devtypeid) {
                dev_type_name = dev_type.devtypename.as_str();
            }
        }
        dev_type_name
    }

    // 服务器主动向指定 IP 发送数据
    async fn send_data_to_ip(
        &self,
        target_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), Box<dyn Error>> {
        let mut stream = TcpStream::connect(target_addr).await?;
        stream.write_all(data).await?;
        Ok(())
    }
}
pub fn get_sensor_type_enum(addr: &SocketAddr) -> SensorType {
    let mut current_dev_type = BusDevTypeManager::default();
    if let Some(monitor) = MONITORS
        .iter()
        .filter(|x| is_ip_legal(&x.devip))
        .find(|a| a.devip.trim().parse::<IpAddr>().unwrap() == addr.ip())
    {
        if let Some(dev_type) = DEVTYPES.iter().find(|a| a.id == monitor.devtypeid) {
            current_dev_type = dev_type.clone()
        }
    }

    match current_dev_type.id {
        21 => SensorType::TemperatureHumidity,
        22 => SensorType::Oxygen,
        23 => SensorType::DustConcentration,
        24 => SensorType::WindSpeed,
        18 => SensorType::No2Sensor,
        48 => SensorType::CarbonMonoxide,
        _ => SensorType::Unknown,
    }
}

// 处理单个客户端连接的异步函数
async fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    tracker: &mut DeviceStatusTracker,
) -> Result<(), Box<dyn Error>> {
    tracker.device_connected(addr);
    let mut buffer = [0; 1024];
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            // 客户端关闭连接
            tracker.device_disconnected(addr);
            break;
        }
        let data = &buffer[0..n];
        // // 这里可以对传感器数据进行处理，比如解析、存储等
        // println!(
        //     "Received sensor data from {}: {:?}",
        //     addr,
        //     String::from_utf8_lossy(data)
        // );

        // let hex_data = bytes_to_hex(data);
        // println!("Received data in hex: {}", hex_data);

        let sensor_type = get_sensor_type_enum(&addr);
        match sensor_type {
            SensorType::Oxygen => {
                let parser = OxygenSensorParser;
                match process_sensor_data(&parser, &addr.ip(), data) {
                    Ok(data) => {
                        println!("{} | 氧气传感器数据: {:?}", chrono::Local::now(), data);
                        let threshold = OxygenThreshold::new();
                        // 检查温度传感器预警
                        if let Some(warning) = handle_warning(SensorType::Oxygen, &data, &threshold)
                        {
                            println!(
                                "{} | 氧气传感器数据触发预警！预警信息: {:?}",
                                chrono::Local::now(),
                                warning
                            );
                        }
                    }
                    Err(err) => eprintln!("氧气传感器解析错误: {}", err),
                }
            }
            _ => {}
        }

        // dont need to respond
        // stream.write_all(b"Data received").await?;
    }
    Ok(())
}

// 统一处理传感器数据
pub fn process_sensor_data(
    parser: &dyn SensorParser,
    addr: &IpAddr,
    data: &[u8],
) -> Result<SensorData, &'static str> {
    parser.parse(addr, data)
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let tcp_host = env::var("TCP_HOST").expect("TCP_HOST must be set");

    let listener = TcpListener::bind(&tcp_host)
        .await
        .expect("服务器端口监听失败");
    println!("Server listening on {}", tcp_host);

    let whitelist = MONITORS
        .iter()
        .filter(|x| is_ip_legal(&x.devip))
        .map(|x| x.devip.clone().parse().unwrap())
        .collect::<Vec<IpAddr>>();
    // let mut count = 1;

    // for ip in whitelist.iter() {
    //     println!("{}: {}", count, ip);
    //     count += 1;
    // }
    // // 定义 IP 白名单
    // let whitelist = vec![
    //     // "127.0.0.1".parse().unwrap(),
    //      // 可以添加更多允许的 IP 地址
    // ];
    let tracker = DeviceStatusTracker::new(whitelist);

    loop {
        let (stream, addr) = listener.accept().await?;
        if tracker.is_ip_allowed(addr) {
            let mut tracker_clone = tracker.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, &mut tracker_clone).await {
                    eprintln!("Error handling connection from {}: {}", addr, e);
                    tracker_clone.device_disconnected(addr);
                }
            });
        } else {
            println!("Connection from {} rejected: IP not in whitelist", addr);
        }
    }
}
/// 判断ip是否合法
pub fn is_ip_legal(ip: &str) -> bool {
    match ip.parse::<IpAddr>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
