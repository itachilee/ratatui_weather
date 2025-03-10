use crate::modbus::monitor_parser::SensorData;
use chrono::{DateTime, Local};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
// 定义传感器类型枚举
// #[derive(Debug)]
pub enum SensorType {
    Unknown = 0,

    // 温湿度,接入两个传感器
    TemperatureHumidity = 21,
    // 氧气
    Oxygen = 22,
    // 粉尘浓度
    DustConcentration = 23,
    // 风速
    WindSpeed = 24,
    // 二氧化氮
    No2Sensor = 18,
    // 一氧化碳
    CarbonMonoxide = 48,
}
impl Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorType::Unknown => write!(f, "未知"),
            SensorType::TemperatureHumidity => write!(f, "温湿度"),
            SensorType::Oxygen => write!(f, "氧气"),
            SensorType::DustConcentration => write!(f, "粉尘浓度"),
            SensorType::WindSpeed => write!(f, "风速"),
            SensorType::No2Sensor => write!(f, "二氧化氮"),
            SensorType::CarbonMonoxide => write!(f, "一氧化碳"),
        }
    }
}

// 让 `Debug` 输出与 `Display` 一致（可选）
impl Debug for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
// 定义预警原因枚举
#[derive(Debug)]
pub enum WarningReason {
    AboveThreshold,
    BelowThreshold,
}
impl Display for WarningReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WarningReason::AboveThreshold => write!(f, "超过阈值"),
            WarningReason::BelowThreshold => write!(f, "低于阈值"),
        }
    }
}
// 定义预警信息结构体
#[derive(Debug)]
pub struct WarningInfo {
    pub sensor_type: SensorType,
    pub dev_ip: String,
    pub value: f64,
    pub threshold: f64,
    pub reason: WarningReason,
    pub timestamp: DateTime<Local>,
}

impl std::fmt::Display for WarningInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}| {}传感器设备数据【{}】{}【{}】触发预警！请及时进行处理!",
            // self.timestamp,
            self.dev_ip,
            self.sensor_type.to_string(),
            self.value,
            self.reason.to_string(),
            self.threshold,
        )
    }
}
// 定义温度传感器阈值结构体
#[derive(Debug)]
pub struct TemperatureThreshold {
    pub min: f64,
    pub max: f64,
}

impl TemperatureThreshold {
    fn new() -> Self {
        Self {
            min: 10.0,
            max: 34.0,
        }
    }
}

// // 温度传感器预警规则检查函数
// fn check_temperature_warning(data: &SensorData, threshold: &TemperatureThreshold) -> bool {
//     let temp = data.value;
//     temp < threshold.min || temp > threshold.max
// }

// 温度传感器预警规则检查函数
fn check_temperature_warning(
    data: &SensorData,
    threshold: &TemperatureThreshold,
) -> Option<WarningInfo> {
    let temp = data.value;
    if temp < threshold.min {
        Some(WarningInfo {
            sensor_type: SensorType::TemperatureHumidity,
            dev_ip: data.dev_ip.clone(),
            value: temp,
            threshold: threshold.min,
            reason: WarningReason::BelowThreshold,
            timestamp: Local::now(),
        })
    } else if temp > threshold.max {
        Some(WarningInfo {
            sensor_type: SensorType::TemperatureHumidity,
            dev_ip: data.dev_ip.clone(),
            value: temp,
            threshold: threshold.max,
            reason: WarningReason::AboveThreshold,
            timestamp: Local::now(),
        })
    } else {
        None
    }
}

// 定义温度传感器阈值结构体
#[derive(Debug)]
pub struct HumidityThreshold {
    pub min: f64,
    pub max: f64,
}

impl HumidityThreshold {
    fn new() -> Self {
        Self {
            min: 10.0,
            max: 34.0,
        }
    }
}

// 温度传感器预警规则检查函数
fn check_humidty_warning(data: &SensorData, threshold: &HumidityThreshold) -> Option<WarningInfo> {
    let temp = data.value;
    if temp < threshold.min {
        Some(WarningInfo {
            sensor_type: SensorType::TemperatureHumidity,
            dev_ip: data.dev_ip.clone(),
            value: temp,
            threshold: threshold.min,
            reason: WarningReason::BelowThreshold,
            timestamp: Local::now(),
        })
    } else if temp > threshold.max {
        Some(WarningInfo {
            sensor_type: SensorType::TemperatureHumidity,
            dev_ip: data.dev_ip.clone(),
            value: temp,
            threshold: threshold.max,
            reason: WarningReason::AboveThreshold,
            timestamp: Local::now(),
        })
    } else {
        None
    }
}

// 定义氧气浓度传感器阈值结构体
#[derive(Debug)]
pub struct OxygenThreshold {
    pub min: f64,
    pub max: f64,
}
impl OxygenThreshold {
    pub fn new() -> Self {
        Self {
            min: 18.0,
            max: 21.0,
        }
    }
}
impl OxygenThreshold {}
// 氧气浓度传感器预警规则检查函数
// 氧气浓度传感器预警规则检查函数
fn check_oxygen_warning(data: &SensorData, threshold: &OxygenThreshold) -> Option<WarningInfo> {
    let value = data.value;
    if value < threshold.min {
        Some(WarningInfo {
            sensor_type: SensorType::Oxygen,
            dev_ip: data.dev_ip.clone(),
            value: value,
            threshold: threshold.min,
            reason: WarningReason::BelowThreshold,
            timestamp: Local::now(),
        })
    } else if value > threshold.max {
        Some(WarningInfo {
            sensor_type: SensorType::Oxygen,
            dev_ip: data.dev_ip.clone(),
            value: value,
            threshold: threshold.max,
            reason: WarningReason::AboveThreshold,
            timestamp: Local::now(),
        })
    } else {
        None
    }
}

// 二氧化氮阈值结构体
#[derive(Debug)]
pub struct No2Threshold {
    pub max: f64,
}
impl No2Threshold {
    fn new() -> Self {
        Self { max: 2.5 }
    }
}
// 二氧化氮浓度传感器预警规则检查函数
fn check_no2_warning(data: &SensorData, threshold: &No2Threshold) -> bool {
    let value = data.value;
    value > threshold.max
}
// 粉尘浓度阈值结构体
#[derive(Debug)]
pub struct DustConcentrationThreshold {
    pub max: f64,
}
impl DustConcentrationThreshold {
    fn new() -> Self {
        Self { max: 4000. }
    }
}
// 粉尘浓度传感器预警规则检查函数
fn check_dust_concentration_warning(
    data: &SensorData,
    threshold: &DustConcentrationThreshold,
) -> bool {
    let value = data.value;
    value > threshold.max
}

// 统一预警处理函数
pub fn handle_warning(
    sensor_type: SensorType,
    data: &SensorData,
    threshold: &impl WarningThreshold,
) -> Option<WarningInfo> {
    match sensor_type {
        SensorType::TemperatureHumidity => {
            let temp_threshold = threshold
                .as_any()
                .downcast_ref::<TemperatureThreshold>()
                .unwrap();
            check_temperature_warning(data, temp_threshold)
        }
        SensorType::Oxygen => {
            let oxygen_threshold = threshold
                .as_any()
                .downcast_ref::<OxygenThreshold>()
                .unwrap();
            check_oxygen_warning(data, oxygen_threshold)
        }
        _ => None,
    }
}

// 预警阈值 trait
pub trait WarningThreshold {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl WarningThreshold for TemperatureThreshold {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl WarningThreshold for OxygenThreshold {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
