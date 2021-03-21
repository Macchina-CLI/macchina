use cfg_if::cfg_if;

#[macro_use]
extern crate lazy_static;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub type BatteryReadout = linux::LinuxBatteryReadout;
        pub type KernelReadout = linux::LinuxKernelReadout;
        pub type MemoryReadout = linux::LinuxMemoryReadout;
        pub type GeneralReadout = linux::LinuxGeneralReadout;
        pub type ProductReadout = linux::LinuxProductReadout;
        pub type PackageReadout = linux::LinuxPackageReadout;
    } else if #[cfg(target_os = "macos")] {
        mod macos;

        pub type BatteryReadout = macos::MacOSBatteryReadout;
        pub type KernelReadout = macos::MacOSKernelReadout;
        pub type MemoryReadout = macos::MacOSMemoryReadout;
        pub type GeneralReadout = macos::MacOSGeneralReadout;
        pub type ProductReadout = macos::MacOSProductReadout;
        pub type PackageReadout = macos::MacOSPackageReadout;
    } else if #[cfg(target_os = "netbsd")] {
        mod netbsd;

        pub type BatteryReadout = netbsd::NetBSDBatteryReadout;
        pub type KernelReadout = netbsd::NetBSDKernelReadout;
        pub type MemoryReadout = netbsd::NetBSDMemoryReadout;
        pub type GeneralReadout = netbsd::NetBSDGeneralReadout;
        pub type ProductReadout = netbsd::NetBSDProductReadout;
        pub type PackageReadout = netbsd::NetBSDPackageReadout;
    } else if #[cfg(target_os = "windows")] {
        mod windows;

        pub type BatteryReadout = windows::WindowsBatteryReadout;
        pub type KernelReadout = windows::WindowsKernelReadout;
        pub type MemoryReadout = windows::WindowsMemoryReadout;
        pub type GeneralReadout = windows::WindowsGeneralReadout;
        pub type ProductReadout = windows::WindowsProductReadout;
        pub type PackageReadout = windows::WindowsPackageReadout;
    } else {
        compiler_error!("This OS is currently not supported by macchina.");
    }
}

pub struct Readouts {
    pub battery: BatteryReadout,
    pub kernel: KernelReadout,
    pub memory: MemoryReadout,
    pub general: GeneralReadout,
    pub product: ProductReadout,
    pub packages: PackageReadout,
}

pub mod extra;
mod shared;
pub mod traits;
