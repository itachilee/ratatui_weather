use crate::db::connection::*;
use crate::db::models::*;
use crate::db::schema::busdevtypemanager::dsl::*;
use crate::db::schema::busmonitormanager::dsl::*;
use diesel::prelude::*;
use once_cell::sync::Lazy;

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

// 新的传感器解析器
pub struct No2SensorParser;

impl SensorParser for No2SensorParser {
    fn parse(&self, data: &[u8]) -> Result<SensorData, &'static str> {
        // 假设 HeaderByteCount 为 0，DataLength 为 2，你可以根据实际情况修改
        const HEADER_BYTE_COUNT: usize = 0;
        const DATA_LENGTH: usize = 2;

        // 数据值
        let data1 = &data[HEADER_BYTE_COUNT..HEADER_BYTE_COUNT + DATA_LENGTH];

        // 将字节数组转换为十六进制字符串
        let wind_speed_hex = bytes_to_hex(data1);

        // 将十六进制字符串转换为 i16
        let wind_speed = match i16::from_str_radix(&wind_speed_hex, 16) {
            Ok(val) => val,
            Err(_) => return Err("Failed to convert hex string to i16"),
        };

        let mut v1 = 0.0;
        if wind_speed != 0 {
            // 首先，将数据转换为浮点数
            let a = wind_speed as f64;
            // 接下来，将 a 除以 10 的 1 次方，以将小数点移动到正确的位置
            v1 = a / 10.0;
        }

        Ok(SensorData {
            sensor_type: "No2传感器".to_string(),
            value: v1,
            unit: "ppm".to_string(),
            status: "正常".to_string(),
        })
    }
}

// 将字节数据转换为十六进制字符串
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in bytes {
        hex_string.push_str(&format!("{:02x}", byte));
    }
    hex_string
}

// 一氧化碳传感器解析器
pub struct CarbonMonoxideSensorParser;
impl SensorParser for CarbonMonoxideSensorParser {
    fn parse(&self, data: &[u8]) -> Result<SensorData, &'static str> {
        // 检查数据长度是否符合协议要求，应答帧至少 7 字节（地址码 1 + 功能码 1 + 有效字节数 1 + CO 值 2 + 校验码 2）
        if data.len() < 7 {
            return Err("Invalid data length for carbon monoxide sensor");
        }

        // 提取 CO 值的字节数据，从第 4 个字节开始取 2 个字节
        let co_value_bytes = &data[3..5];

        // 将 CO 值的字节数据转换为 u16 类型
        let co_value_raw = u16::from_be_bytes([co_value_bytes[0], co_value_bytes[1]]);

        // 计算最终的 CO 值，除以 10 得到实际的 ppm 值
        let co_value = co_value_raw as f64 / 10.0;

        Ok(SensorData {
            value: co_value,
            sensor_type: "CO传感器".to_string(),
            unit: "ppm".to_string(),
            status: "正常".to_string(),
        })
    }
}

// 氧气浓度传感器解析器
pub struct OxygenSensorParser;

impl SensorParser for OxygenSensorParser {
    fn parse(&self, data: &[u8]) -> Result<SensorData, &'static str> {
        // 检查数据长度是否符合要求，应答帧至少 7 字节（地址码 1 + 功能码 1 + 有效字节数 1 + 氧气值 2 + 校验码 2）
        if data.len() < 7 {
            return Err("Invalid data length for oxygen sensor");
        }

        // 提取氧气浓度值的字节数据，从第 4 个字节开始取 2 个字节
        let oxygen_value_bytes = &data[3..5];

        // 将氧气值的字节数据转换为 u16 类型
        let oxygen_value_raw = u16::from_be_bytes([oxygen_value_bytes[0], oxygen_value_bytes[1]]);

        // 计算最终的氧气浓度值，除以 10 得到实际的值
        let oxygen_value = oxygen_value_raw as f64 / 10.0;

        Ok(SensorData {
            sensor_type: "氧气浓度传感器".to_string(),
            value: oxygen_value,
            unit: "%".to_string(),
            status: "正常".to_string(),
        })
    }
}

// 粉尘浓度传感器解析器
pub struct DustConcentrationSensorParser;

impl SensorParser for DustConcentrationSensorParser {
    fn parse(&self, data: &[u8]) -> Result<SensorData, &'static str> {
        // 检查数据长度是否符合要求，应答帧至少 7 字节（地址码 1 + 功能码 1 + 有效字节数 1 + 粉尘值 2 + 校验码 2）
        if data.len() < 7 {
            return Err("Invalid data length for dust concentration sensor");
        }

        // 提取粉尘浓度值的字节数据，从第 4 个字节开始取 2 个字节
        let dust_value_bytes = &data[3..5];

        // 将粉尘值的字节数据转换为 u16 类型
        let dust_value_raw = u16::from_be_bytes([dust_value_bytes[0], dust_value_bytes[1]]);

        // 将 u16 类型的粉尘值转换为 f64 类型
        let dust_value = dust_value_raw as f64;

        Ok(SensorData {
            sensor_type: "粉尘浓度传感器".to_string(),
            value: dust_value,
            unit: "ug/m³".to_string(),
            status: "正常".to_string(),
        })
    }
}
