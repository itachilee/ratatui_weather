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

pub struct Monitor;

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
pub struct DecoderResult {
    pub devip: String,
    pub v1: f64,
    pub v2: f64,
}
trait SensorDecoder {
    fn decoder(data: &[u8]) {}
}
// 风速传感器
pub struct WindSpeed;

impl WindSpeed {
    // 解析从机回复的数据
    pub fn parse_wind_speed_sensor_data(
        &self,
        data: &[u8],
    ) -> Result<(u16, u8, u16, u8), &'static str> {
        // 提取数据部分
        let value_bytes = &data[3..5];
        let decimal_bytes = &data[5..7];
        let device_type_bytes = &data[7..9];
        let status_bytes = &data[9..11];

        // 转换数据为对应的数值
        let value = u16::from_be_bytes([value_bytes[0], value_bytes[1]]);
        let decimal = decimal_bytes[1];
        let device_type = u16::from_be_bytes([device_type_bytes[0], device_type_bytes[1]]);
        let status = status_bytes[1];

        Ok((value, decimal, device_type, status))
    }

    // 将解析结果进行格式化输出
    fn format_parsed_data(&self, value: u16, decimal: u8, device_type: u16, status: u8) {
        // 根据小数点位数进行换算
        let converted_value = value as f64 / (10.0f64.powi(decimal as i32));

        // 解读运行状态
        let status_text = match status {
            0 => "启动中",
            1 => "正常",
            2 => "调校",
            3 => "故障",
            _ => "未知状态",
        };

        println!("采集数据: {:.1}℃", converted_value);
        println!("小数点位数: {}", decimal);
        println!("风速: {:04x} ", device_type);
        println!("运行状态: {}", status_text);
    }

    pub fn print_result(&self, data: &[u8]) {
        match self.parse_wind_speed_sensor_data(data) {
            Ok((value, decimal, device_type, status)) => {
                self.format_parsed_data(value, decimal, device_type, status);
            }
            Err(err) => {
                eprintln!("解析错误: {}", err);
            }
        }
    }
}

pub struct Temperature;

impl Temperature {
    fn parse_temperature_and_humidity(&self, data: &[u8]) -> Result<(f64, f64), &'static str> {
        // 检查数据长度是否符合协议要求
        if data.len() != 9 {
            return Err("Invalid data length");
        }

        // 提取湿度和温度的字节数据
        let humidity_bytes = [data[3], data[4]];
        let temperature_bytes = [data[5], data[6]];

        // 将湿度字节转换为 u16 类型
        let humidity_raw = u16::from_be_bytes(humidity_bytes);
        // 计算湿度值，乘以系数 0.1
        let humidity = humidity_raw as f64 * 0.1;

        // 将温度字节转换为 i16 类型（考虑负数情况）
        let temperature_raw = i16::from_be_bytes(temperature_bytes);
        // 计算温度值，乘以系数 0.1
        let temperature = temperature_raw as f64 * 0.1;

        Ok((temperature, humidity))
    }
}

// 定义统一的数据格式
#[derive(Debug)]
pub struct SensorData {
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub status: String,
}

// 定义传感器解析 trait
pub trait SensorParser {
    fn parse(&self, data: &[u8]) -> Result<SensorData, &'static str>;
}

// 风速传感器解析器
pub struct WindSpeedSensorParser;

impl SensorParser for WindSpeedSensorParser {
    fn parse(&self, data: &[u8]) -> Result<SensorData, &'static str> {
        let value_bytes = &data[3..5];
        let decimal_bytes = &data[5..7];
        let status_bytes = &data[9..11];

        let value = u16::from_be_bytes([value_bytes[0], value_bytes[1]]);
        let decimal = decimal_bytes[1];
        let status = status_bytes[1];

        let converted_value = value as f64 / (10.0f64.powi(decimal as i32));
        let status_text = match status {
            0 => "启动中",
            1 => "正常",
            2 => "调校",
            3 => "故障",
            _ => "未知状态",
        };

        Ok(SensorData {
            sensor_type: "风速传感器".to_string(),
            value: converted_value,
            unit: "m/s".to_string(),
            status: status_text.to_string(),
        })
    }
}

// 温湿度传感器解析器
pub struct TemperatureHumiditySensorParser;

impl SensorParser for TemperatureHumiditySensorParser {
    fn parse(&self, data: &[u8]) -> Result<SensorData, &'static str> {
        let humidity_bytes = [data[3], data[4]];
        let temperature_bytes = [data[5], data[6]];

        let humidity_raw = u16::from_be_bytes(humidity_bytes);
        let humidity = humidity_raw as f64 * 0.1;
        let temperature_raw = i16::from_be_bytes(temperature_bytes);
        let temperature = temperature_raw as f64 * 0.1;

        // 这里为了简化，只返回温度数据，可根据需求扩展返回湿度数据
        Ok(SensorData {
            sensor_type: "温湿度传感器".to_string(),
            value: temperature,
            unit: "℃".to_string(),
            status: "正常".to_string(),
        })
    }
}
