use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

/// Compute type for Pod resources.
///
/// Determines whether a Pod will have GPU or CPU compute resources attached.
/// When set to `GPU`, the Pod will have GPU resources and GPU-related properties
/// will be considered. When set to `CPU`, only CPU-related properties will be used.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum ComputeType {
    /// GPU-based compute resources.
    #[default]
    Gpu,
    /// CPU-based compute resources.
    Cpu,
}

/// RunPod cloud deployment type.
///
/// Determines which RunPod cloud environment the Pod will be deployed to.
/// Secure Cloud offers guaranteed availability and enterprise features,
/// while Community Cloud offers lower costs with potentially less reliability.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum CloudType {
    /// Secure Cloud deployment with guaranteed resources and enterprise features.
    #[default]
    Secure,
    /// Community Cloud deployment with lower costs and shared resources.
    Community,
}

/// Current operational status of a Pod.
///
/// Represents the lifecycle state of a Pod, indicating whether it's actively
/// running, has exited gracefully, or has been forcibly terminated.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum PodStatus {
    /// Pod is currently running and operational.
    #[default]
    Running,
    /// Pod has finished execution and exited normally.
    Exited,
    /// Pod has been forcibly terminated or stopped.
    Terminated,
}

/// Available CUDA versions for GPU Pods.
///
/// Specifies which CUDA runtime version should be available on the GPU Pod.
/// This is only relevant for GPU Pods and determines software compatibility.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CudaVersion {
    /// CUDA version 13.0.
    #[serde(rename = "13.0")]
    V13_0,
    /// CUDA version 12.9.
    #[serde(rename = "12.9")]
    V12_9,
    /// CUDA version 12.8.
    #[serde(rename = "12.8")]
    V12_8,
    /// CUDA version 12.7.
    #[serde(rename = "12.7")]
    V12_7,
    /// CUDA version 12.6.
    #[serde(rename = "12.6")]
    V12_6,
    /// CUDA version 12.5.
    #[serde(rename = "12.5")]
    V12_5,
    /// CUDA version 12.4.
    #[serde(rename = "12.4")]
    V12_4,
    /// CUDA version 12.3.
    #[serde(rename = "12.3")]
    V12_3,
    /// CUDA version 12.2.
    #[serde(rename = "12.2")]
    V12_2,
    /// CUDA version 12.1.
    #[serde(rename = "12.1")]
    V12_1,
    /// CUDA version 12.0.
    #[serde(rename = "12.0")]
    #[default]
    V12_0,
    /// CUDA version 11.8.
    #[serde(rename = "11.8")]
    V11_8,
}

/// Available GPU hardware types for GPU Pods.
///
/// Represents the specific GPU models that can be attached to a Pod.
/// Each GPU type has different performance characteristics, memory capacity,
/// and pricing. The availability of each type varies by data center and time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GpuTypeId {
    /// NVIDIA GeForce RTX 4090.
    #[serde(rename = "NVIDIA GeForce RTX 4090")]
    NvidiaGeForceRtx4090,
    /// NVIDIA A40.
    #[serde(rename = "NVIDIA A40")]
    NvidiaA40,
    /// NVIDIA RTX A5000.
    #[serde(rename = "NVIDIA RTX A5000")]
    NvidiaRtxA5000,
    /// NVIDIA GeForce RTX 3090.
    #[serde(rename = "NVIDIA GeForce RTX 3090")]
    NvidiaGeForceRtx3090,
    /// NVIDIA RTX A4500.
    #[serde(rename = "NVIDIA RTX A4500")]
    NvidiaRtxA4500,
    /// NVIDIA RTX A6000.
    #[serde(rename = "NVIDIA RTX A6000")]
    NvidiaRtxA6000,
    /// NVIDIA L40S.
    #[serde(rename = "NVIDIA L40S")]
    NvidiaL40S,
    /// NVIDIA L4.
    #[serde(rename = "NVIDIA L4")]
    NvidiaL4,
    /// NVIDIA H100 80GB HBM3.
    #[serde(rename = "NVIDIA H100 80GB HBM3")]
    NvidiaH100_80GbHbm3,
    /// NVIDIA RTX 4000 Ada Generation.
    #[serde(rename = "NVIDIA RTX 4000 Ada Generation")]
    NvidiaRtx4000Ada,
    /// NVIDIA A100 80GB PCIe.
    #[serde(rename = "NVIDIA A100 80GB PCIe")]
    NvidiaA100_80GbPcie,
    /// NVIDIA A100-SXM4-80GB.
    #[serde(rename = "NVIDIA A100-SXM4-80GB")]
    NvidiaA100Sxm4_80Gb,
    /// NVIDIA A100-SXM4-40GB.
    #[serde(rename = "NVIDIA A100-SXM4-40GB")]
    NvidiaA100Sxm4_40Gb,
    /// NVIDIA RTX A4000.
    #[serde(rename = "NVIDIA RTX A4000")]
    NvidiaRtxA4000,
    /// NVIDIA RTX 6000 Ada Generation.
    #[serde(rename = "NVIDIA RTX 6000 Ada Generation")]
    NvidiaRtx6000Ada,
    /// NVIDIA RTX 2000 Ada Generation.
    #[serde(rename = "NVIDIA RTX 2000 Ada Generation")]
    NvidiaRtx2000Ada,
    /// NVIDIA H200.
    #[serde(rename = "NVIDIA H200")]
    NvidiaH200,
    /// NVIDIA L40.
    #[serde(rename = "NVIDIA L40")]
    NvidiaL40,
    /// NVIDIA H100 NVL.
    #[serde(rename = "NVIDIA H100 NVL")]
    NvidiaH100Nvl,
    /// NVIDIA H100 PCIe.
    #[serde(rename = "NVIDIA H100 PCIe")]
    NvidiaH100Pcie,
    /// NVIDIA GeForce RTX 3080 Ti.
    #[serde(rename = "NVIDIA GeForce RTX 3080 Ti")]
    NvidiaGeForceRtx3080Ti,
    /// NVIDIA GeForce RTX 3080.
    #[serde(rename = "NVIDIA GeForce RTX 3080")]
    NvidiaGeForceRtx3080,
    /// NVIDIA GeForce RTX 3070.
    #[serde(rename = "NVIDIA GeForce RTX 3070")]
    NvidiaGeForceRtx3070,
    /// Tesla V100-PCIE-16GB.
    #[serde(rename = "Tesla V100-PCIE-16GB")]
    TeslaV100Pcie16Gb,
    /// Tesla V100-PCIE-32GB.
    #[serde(rename = "Tesla V100-PCIE-32GB")]
    TeslaV100Pcie32Gb,
    /// Tesla T4.
    #[serde(rename = "Tesla T4")]
    TeslaT4,
    /// AMD Instinct MI300X OAM.
    #[serde(rename = "AMD Instinct MI300X OAM")]
    AmdInstinctMi300XOam,
    /// NVIDIA RTX A2000.
    #[serde(rename = "NVIDIA RTX A2000")]
    NvidiaRtxA2000,
    /// NVIDIA RTX A30.
    #[serde(rename = "NVIDIA RTX A30")]
    NvidiaRtxA30,
    /// Tesla V100-FHHL-16GB.
    #[serde(rename = "Tesla V100-FHHL-16GB")]
    TeslaV100Fhhl16Gb,
    /// NVIDIA GeForce RTX 4080 SUPER.
    #[serde(rename = "NVIDIA GeForce RTX 4080 SUPER")]
    NvidiaGeForceRtx4080Super,
    /// Tesla V100-SXM2-16GB.
    #[serde(rename = "Tesla V100-SXM2-16GB")]
    TeslaV100Sxm2_16Gb,
    /// NVIDIA GeForce RTX 4070 Ti.
    #[serde(rename = "NVIDIA GeForce RTX 4070 Ti")]
    NvidiaGeForceRtx4070Ti,
    /// Tesla V100-SXM2-32GB.
    #[serde(rename = "Tesla V100-SXM2-32GB")]
    TeslaV100Sxm2_32Gb,
    /// NVIDIA RTX 4000 SFF Ada Generation.
    #[serde(rename = "NVIDIA RTX 4000 SFF Ada Generation")]
    NvidiaRtx4000SffAda,
    /// NVIDIA RTX 5000 Ada Generation.
    #[serde(rename = "NVIDIA RTX 5000 Ada Generation")]
    NvidiaRtx5000Ada,
    /// NVIDIA GeForce RTX 5090.
    #[serde(rename = "NVIDIA GeForce RTX 5090")]
    NvidiaGeForceRtx5090,
    /// NVIDIA A30.
    #[serde(rename = "NVIDIA A30")]
    NvidiaA30,
    /// NVIDIA GeForce RTX 4080.
    #[serde(rename = "NVIDIA GeForce RTX 4080")]
    NvidiaGeForceRtx4080,
    /// NVIDIA GeForce RTX 5080.
    #[serde(rename = "NVIDIA GeForce RTX 5080")]
    NvidiaGeForceRtx5080,
    /// NVIDIA GeForce RTX 3090 Ti.
    #[serde(rename = "NVIDIA GeForce RTX 3090 Ti")]
    NvidiaGeForceRtx3090Ti,
    /// NVIDIA B200.
    #[serde(rename = "NVIDIA B200")]
    NvidiaB200,
    /// Unknown or unsupported GPU hardware type.
    #[default]
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for GpuTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Unknown => "Unknown",
            Self::NvidiaGeForceRtx4090 => "NVIDIA GeForce RTX 4090",
            Self::NvidiaA40 => "NVIDIA A40",
            Self::NvidiaRtxA5000 => "NVIDIA RTX A5000",
            Self::NvidiaGeForceRtx3090 => "NVIDIA GeForce RTX 3090",
            Self::NvidiaRtxA4500 => "NVIDIA RTX A4500",
            Self::NvidiaRtxA6000 => "NVIDIA RTX A6000",
            Self::NvidiaL40S => "NVIDIA L40S",
            Self::NvidiaL4 => "NVIDIA L4",
            Self::NvidiaH100_80GbHbm3 => "NVIDIA H100 80GB HBM3",
            Self::NvidiaRtx4000Ada => "NVIDIA RTX 4000 Ada Generation",
            Self::NvidiaA100_80GbPcie => "NVIDIA A100 80GB PCIe",
            Self::NvidiaA100Sxm4_80Gb => "NVIDIA A100-SXM4-80GB",
            Self::NvidiaA100Sxm4_40Gb => "NVIDIA A100-SXM4-40GB",
            Self::NvidiaRtxA4000 => "NVIDIA RTX A4000",
            Self::NvidiaRtx6000Ada => "NVIDIA RTX 6000 Ada Generation",
            Self::NvidiaRtx2000Ada => "NVIDIA RTX 2000 Ada Generation",
            Self::NvidiaH200 => "NVIDIA H200",
            Self::NvidiaL40 => "NVIDIA L40",
            Self::NvidiaH100Nvl => "NVIDIA H100 NVL",
            Self::NvidiaH100Pcie => "NVIDIA H100 PCIe",
            Self::NvidiaGeForceRtx3080Ti => "NVIDIA GeForce RTX 3080 Ti",
            Self::NvidiaGeForceRtx3080 => "NVIDIA GeForce RTX 3080",
            Self::NvidiaGeForceRtx3070 => "NVIDIA GeForce RTX 3070",
            Self::TeslaV100Pcie16Gb => "Tesla V100-PCIE-16GB",
            Self::TeslaV100Pcie32Gb => "Tesla V100-PCIE-32GB",
            Self::TeslaT4 => "Tesla T4",
            Self::AmdInstinctMi300XOam => "AMD Instinct MI300X OAM",
            Self::NvidiaRtxA2000 => "NVIDIA RTX A2000",
            Self::NvidiaRtxA30 => "NVIDIA RTX A30",
            Self::TeslaV100Fhhl16Gb => "Tesla V100-FHHL-16GB",
            Self::NvidiaGeForceRtx4080Super => "NVIDIA GeForce RTX 4080 SUPER",
            Self::TeslaV100Sxm2_16Gb => "Tesla V100-SXM2-16GB",
            Self::NvidiaGeForceRtx4070Ti => "NVIDIA GeForce RTX 4070 Ti",
            Self::TeslaV100Sxm2_32Gb => "Tesla V100-SXM2-32GB",
            Self::NvidiaRtx4000SffAda => "NVIDIA RTX 4000 SFF Ada Generation",
            Self::NvidiaRtx5000Ada => "NVIDIA RTX 5000 Ada Generation",
            Self::NvidiaGeForceRtx5090 => "NVIDIA GeForce RTX 5090",
            Self::NvidiaA30 => "NVIDIA A30",
            Self::NvidiaGeForceRtx4080 => "NVIDIA GeForce RTX 4080",
            Self::NvidiaGeForceRtx5080 => "NVIDIA GeForce RTX 5080",
            Self::NvidiaGeForceRtx3090Ti => "NVIDIA GeForce RTX 3090 Ti",
            Self::NvidiaB200 => "NVIDIA B200",
        };
        write!(f, "{}", s)
    }
}

/// Available CPU flavor configurations for CPU Pods.
///
/// Represents different CPU configurations available for CPU-only Pods.
/// Each flavor provides different combinations of cores, memory, and performance
/// characteristics optimized for various workload types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CpuFlavorId {
    /// 3rd generation CPU configuration - compute optimized.
    Cpu3c,
    /// 3rd generation CPU configuration - general purpose.
    Cpu3g,
    /// 3rd generation CPU configuration - memory optimized.
    Cpu3m,
    /// 5th generation CPU configuration - compute optimized.
    Cpu5c,
    /// 5th generation CPU configuration - general purpose.
    Cpu5g,
    /// 5th generation CPU configuration - memory optimized.
    Cpu5m,
    /// Unknown or unsupported CPU flavor.
    #[default]
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for CpuFlavorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Unknown => "unknown",
            Self::Cpu3c => "cpu3c",
            Self::Cpu3g => "cpu3g",
            Self::Cpu3m => "cpu3m",
            Self::Cpu5c => "cpu5c",
            Self::Cpu5g => "cpu5g",
            Self::Cpu5m => "cpu5m",
        };
        write!(f, "{}", s)
    }
}

/// RunPod data center locations.
///
/// Represents the geographic locations where RunPod has data centers.
/// The choice of data center affects latency, regulatory compliance,
/// and resource availability. Costs may also vary by location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DataCenterId {
    /// Romania (EU-RO-1).
    #[serde(rename = "EU-RO-1")]
    EuRo1,
    /// Montreal, Canada (CA-MTL-1).
    #[serde(rename = "CA-MTL-1")]
    CaMtl1,
    /// Sweden (EU-SE-1).
    #[serde(rename = "EU-SE-1")]
    EuSe1,
    /// Illinois, USA (US-IL-1).
    #[serde(rename = "US-IL-1")]
    UsIl1,
    /// Iceland (EUR-IS-1).
    #[serde(rename = "EUR-IS-1")]
    EurIs1,
    /// Czech Republic (EU-CZ-1).
    #[serde(rename = "EU-CZ-1")]
    EuCz1,
    /// Texas, USA (US-TX-3).
    #[serde(rename = "US-TX-3")]
    UsTx3,
    /// Iceland (EUR-IS-2).
    #[serde(rename = "EUR-IS-2")]
    EurIs2,
    /// Kansas, USA (US-KS-2).
    #[serde(rename = "US-KS-2")]
    UsKs2,
    /// Georgia, USA (US-GA-2).
    #[serde(rename = "US-GA-2")]
    UsGa2,
    /// Washington, USA (US-WA-1).
    #[serde(rename = "US-WA-1")]
    UsWa1,
    /// Texas, USA (US-TX-1).
    #[serde(rename = "US-TX-1")]
    UsTx1,
    /// Montreal, Canada (CA-MTL-3).
    #[serde(rename = "CA-MTL-3")]
    CaMtl3,
    /// Netherlands (EU-NL-1).
    #[serde(rename = "EU-NL-1")]
    EuNl1,
    /// Texas, USA (US-TX-4).
    #[serde(rename = "US-TX-4")]
    UsTx4,
    /// California, USA (US-CA-2).
    #[serde(rename = "US-CA-2")]
    UsCa2,
    /// North Carolina, USA (US-NC-1).
    #[serde(rename = "US-NC-1")]
    UsNc1,
    /// Australia (OC-AU-1).
    #[serde(rename = "OC-AU-1")]
    OcAu1,
    /// Delaware, USA (US-DE-1).
    #[serde(rename = "US-DE-1")]
    UsDe1,
    /// Iceland (EUR-IS-3).
    #[serde(rename = "EUR-IS-3")]
    EurIs3,
    /// Montreal, Canada (CA-MTL-2).
    #[serde(rename = "CA-MTL-2")]
    CaMtl2,
    /// Japan (AP-JP-1).
    #[serde(rename = "AP-JP-1")]
    ApJp1,
    /// Norway (EUR-NO-1).
    #[serde(rename = "EUR-NO-1")]
    EurNo1,
    /// France (EU-FR-1).
    #[serde(rename = "EU-FR-1")]
    EuFr1,
    /// Kansas, USA (US-KS-3).
    #[serde(rename = "US-KS-3")]
    UsKs3,
    /// Georgia, USA (US-GA-1).
    #[serde(rename = "US-GA-1")]
    UsGa1,
    /// India (AP-IN-1).
    #[serde(rename = "AP-IN-1")]
    ApIn1,
    /// Maryland, USA (US-MD-1).
    #[serde(rename = "US-MD-1")]
    UsMd1,
    /// Unknown or unsupported data center region.
    #[default]
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for DataCenterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Unknown => "Unknown",
            Self::EuRo1 => "EU-RO-1",
            Self::CaMtl1 => "CA-MTL-1",
            Self::EuSe1 => "EU-SE-1",
            Self::UsIl1 => "US-IL-1",
            Self::EurIs1 => "EUR-IS-1",
            Self::EuCz1 => "EU-CZ-1",
            Self::UsTx3 => "US-TX-3",
            Self::EurIs2 => "EUR-IS-2",
            Self::UsKs2 => "US-KS-2",
            Self::UsGa2 => "US-GA-2",
            Self::UsWa1 => "US-WA-1",
            Self::UsTx1 => "US-TX-1",
            Self::CaMtl3 => "CA-MTL-3",
            Self::EuNl1 => "EU-NL-1",
            Self::UsTx4 => "US-TX-4",
            Self::UsCa2 => "US-CA-2",
            Self::UsNc1 => "US-NC-1",
            Self::OcAu1 => "OC-AU-1",
            Self::UsDe1 => "US-DE-1",
            Self::EurIs3 => "EUR-IS-3",
            Self::CaMtl2 => "CA-MTL-2",
            Self::ApJp1 => "AP-JP-1",
            Self::EurNo1 => "EUR-NO-1",
            Self::EuFr1 => "EU-FR-1",
            Self::UsKs3 => "US-KS-3",
            Self::UsGa1 => "US-GA-1",
            Self::ApIn1 => "AP-IN-1",
            Self::UsMd1 => "US-MD-1",
        };
        write!(f, "{}", s)
    }
}

/// Detailed information about GPU resources attached to a Pod.
///
/// Contains comprehensive details about the GPU configuration including
/// hardware specifications, pricing across different billing periods,
/// and availability in different cloud types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuInfo {
    /// Unique identifier for this GPU type.
    pub id: String,
    /// Number of GPUs of this type attached to the Pod.
    pub count: i32,
    /// Human-readable display name for the GPU type.
    pub display_name: String,
    /// Hourly price per GPU in RunPod credits for Secure Cloud.
    pub secure_price: f64,
    /// Hourly price per GPU in RunPod credits for Community Cloud.
    pub community_price: f64,
    /// Monthly rate per GPU in RunPod credits (30-day billing).
    pub one_month_price: f64,
    /// Quarterly rate per GPU in RunPod credits (90-day billing).
    pub three_month_price: f64,
    /// Semi-annual rate per GPU in RunPod credits (180-day billing).
    pub six_month_price: f64,
    /// Weekly rate per GPU in RunPod credits (7-day billing).
    pub one_week_price: f64,
    /// Spot pricing per GPU hour in RunPod credits for Community Cloud.
    pub community_spot_price: f64,
    /// Spot pricing per GPU hour in RunPod credits for Secure Cloud.
    pub secure_spot_price: f64,
}

/// Detailed information about CPU resources for a Pod.
///
/// Contains specifications about the CPU configuration including
/// core count, threading capabilities, and organizational grouping.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuType {
    /// Unique identifier for this CPU type.
    pub id: String,
    /// Human-readable display name for the CPU type.
    pub display_name: String,
    /// Number of physical CPU cores available.
    pub cores: f64,
    /// Number of threads supported per physical core.
    pub threads_per_core: f64,
    /// Group identifier for organizing similar CPU types.
    pub group_id: String,
}

/// Detailed information about the physical machine hosting a Pod.
///
/// Contains comprehensive details about the hardware infrastructure,
/// networking capabilities, pricing, and operational status of the
/// machine where the Pod is running.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    /// Minimum number of GPUs required for Pods on this machine.
    pub min_pod_gpu_count: Option<i32>,
    /// Identifier for the GPU type available on this machine.
    pub gpu_type_id: Option<String>,
    /// Detailed information about the GPU type on this machine.
    pub gpu_type: Option<GpuInfo>,
    /// Total number of CPU cores available on this machine.
    pub cpu_count: Option<i32>,
    /// Identifier for the CPU type on this machine.
    pub cpu_type_id: Option<String>,
    /// Detailed information about the CPU type on this machine.
    pub cpu_type: Option<CpuType>,
    /// Geographic location description of this machine.
    #[serde(default)]
    pub location: String,
    /// Data center identifier where this machine is located.
    #[serde(default)]
    pub data_center_id: String,
    /// Disk I/O throughput capacity in megabytes per second.
    pub disk_throughput_m_bps: Option<i32>,
    /// Maximum network download speed in megabits per second.
    pub max_download_speed_mbps: Option<i32>,
    /// Maximum network upload speed in megabits per second.
    pub max_upload_speed_mbps: Option<i32>,
    /// Whether this machine supports public IP assignment.
    #[serde(default)]
    pub support_public_ip: bool,
    /// Whether this machine is in the Secure Cloud environment.
    #[serde(default)]
    pub secure_cloud: bool,
    /// Scheduled maintenance start time, if any.
    pub maintenance_start: Option<String>,
    /// Scheduled maintenance end time, if any.
    pub maintenance_end: Option<String>,
    /// Additional information about scheduled maintenance.
    pub maintenance_note: Option<String>,
    /// General notes or information about this machine.
    pub note: Option<String>,
    /// Current hourly cost in RunPod credits for this machine.
    #[serde(default)]
    pub cost_per_hr: f64,
    /// Current price per GPU hour in RunPod credits, if applicable.
    pub current_price_per_gpu: Option<f64>,
    /// Number of GPUs currently available on this machine.
    pub gpu_available: Option<i32>,
    /// Human-readable name of the GPU type on this machine.
    pub gpu_display_name: Option<String>,
}

/// A savings plan applied to reduce Pod costs.
///
/// Savings plans offer discounted pricing in exchange for longer-term
/// commitments to specific GPU types. They automatically apply to
/// eligible Pods to reduce the effective hourly cost.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavingsPlan {
    /// Discounted hourly cost per GPU in RunPod credits.
    pub cost_per_hr: f64,
    /// UTC timestamp when this savings plan expires.
    pub end_time: String,
    /// GPU type identifier that this savings plan applies to.
    pub gpu_type_id: String,
    /// Unique identifier for this savings plan.
    pub id: String,
    /// Pod identifier that this savings plan is currently applied to.
    pub pod_id: String,
    /// UTC timestamp when this savings plan became active.
    pub start_time: String,
}

/// A persistent network-attached storage volume.
///
/// Network volumes provide persistent storage that can be shared across
/// multiple Pods and persists beyond individual Pod lifecycles. They are
/// located in specific data centers and can be mounted to Pods in the same region.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolume {
    /// Unique identifier for this network volume.
    pub id: String,
    /// User-defined name for this network volume.
    pub name: String,
    /// Storage capacity of this volume in gigabytes.
    pub size: i32,
    /// Data center where this network volume is located.
    pub data_center_id: String,
}

/// Environment variables for Pod containers.
///
/// A key-value mapping of environment variables that will be set
/// in the Pod's container runtime environment.
pub type EnvVars = HashMap<String, String>;

/// Port mappings from internal to external ports.
///
/// Maps internal container ports (as strings) to external public ports
/// (as integers) for network access to the Pod.
pub type PortMappings = HashMap<String, i32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_deserialization_from_empty_json() {
        let m: Machine = serde_json::from_str("{}").expect("empty machine should work");
        assert_eq!(m.location, "");
        assert_eq!(m.data_center_id, "");
        assert!(!m.support_public_ip);
        assert!(!m.secure_cloud);
        assert_eq!(m.cost_per_hr, 0.0);
    }
}
