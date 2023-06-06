pub mod can_hardware_plugin;
pub use can_hardware_plugin::CanHardwarePluginTrait;

pub mod can_hardware_interface;

#[cfg(feature = "peak_can_driver")]
mod peak;
#[cfg(feature = "peak_can_driver")]
pub use peak::PeakCanDriver as CanDriver;

#[cfg(any(feature = "mock_can_driver"))]
mod mock;
#[cfg(any(feature = "mock_can_driver"))]
pub use mock::MockCanDriver as CanDriver;

#[cfg(all(target_family = "unix", feature = "socket_can_driver"))]
mod socket;
#[cfg(all(target_family = "unix", feature = "socket_can_driver"))]
pub use socket::SocketCanPlugin as CanDriver;

// #[cfg_attr(feature = "peak_can_driver", path = "peak")]
// #[cfg_attr(all(target_family = "unix", feature = "socket_can_driver"), path = "socket")]
// #[cfg_attr(feature = "mock_can_driver", path = "mock")]
// pub use CanPlugin;
