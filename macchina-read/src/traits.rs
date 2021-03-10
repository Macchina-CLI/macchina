use ReadoutError::MetricNotAvailable;

#[derive(Debug)]
pub enum ReadoutError {
    MetricNotAvailable,
    SysctlError(String),
    IoError(String),
    Other(String),
}

pub trait BatteryReadout {
    fn new() -> Self;

    fn percentage(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn status(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
}

pub trait KernelReadout {
    fn new() -> Self;

    fn os_release(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn os_type(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
}

pub trait MemoryReadout {
    fn new() -> Self;

    fn total(&self) -> Result<u64, ReadoutError> { Err(MetricNotAvailable) }
    fn free(&self) -> Result<u64, ReadoutError> { Err(MetricNotAvailable) }
    fn buffers(&self) -> Result<u64, ReadoutError> { Err(MetricNotAvailable) }
    fn cached(&self) -> Result<u64, ReadoutError> { Err(MetricNotAvailable) }
    fn reclaimable(&self) -> Result<u64, ReadoutError> { Err(MetricNotAvailable) }
    fn used(&self) -> Result<u64, ReadoutError> { Err(MetricNotAvailable) }
}

pub trait PackageReadout {
    fn new() -> Self;

    fn count_pkgs(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
}

pub trait ProductReadout {
    fn new() -> Self;

    fn version(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn vendor(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn family(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn name(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn product(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
}

pub trait GeneralReadout {
    fn new() -> Self;

    fn username(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn hostname(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn distribution(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn desktop_environment(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn window_manager(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn terminal(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn shell(&self, shorthand: bool) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn cpu_model_name(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn uptime(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
    fn machine(&self) -> Result<String, ReadoutError> { Err(MetricNotAvailable) }
}