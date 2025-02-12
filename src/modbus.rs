use std::collections::HashMap;
use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

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
        if let Some(status) = self.connected_devices.get_mut(&addr) {
            if !*status {
                println!("Device {} connected", addr);
                *status = true;
            }
        } else {
            println!("Device {} connected", addr);
            self.connected_devices.insert(addr, true);
        }
    }

    // 记录设备断开连接
    fn device_disconnected(&mut self, addr: SocketAddr) {
        if let Some(status) = self.connected_devices.get_mut(&addr) {
            if *status {
                println!("Device {} disconnected", addr);
                *status = false;
            }
        }
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
        // 这里可以对传感器数据进行处理，比如解析、存储等
        println!(
            "Received sensor data from {}: {:?}",
            addr,
            String::from_utf8_lossy(data)
        );

        // 示例：向客户端发送响应
        stream.write_all(b"Data received").await?;
    }
    Ok(())
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    // 定义 IP 白名单
    let whitelist = vec![
        // "127.0.0.1".parse().unwrap(),
        // 可以添加更多允许的 IP 地址
    ];
    let mut tracker = DeviceStatusTracker::new(whitelist);

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
