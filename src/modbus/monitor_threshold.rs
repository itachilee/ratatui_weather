use crate::modbus::monitor_parser::SensorData;
// 定义传感器类型枚举
#[derive(Debug)]
pub enum SensorType {
    Unknown,

    // 温湿度
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

// 温度传感器预警规则检查函数
fn check_temperature_warning(data: &SensorData, threshold: &TemperatureThreshold) -> bool {
    let temp = data.value;
    temp < threshold.min || temp > threshold.max
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
fn check_oxygen_warning(data: &SensorData, threshold: &OxygenThreshold) -> bool {
    let value = data.value;
    value < threshold.min || value > threshold.max
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
) -> bool {
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
        _ => false,
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
