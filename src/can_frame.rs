//================================================================================================
/// @file can_frame.rs
///
/// @brief A classical CAN frame, with 8 data bytes
/// @author Thom de Jong
///
/// @copyright 2023 Thom de Jong
//================================================================================================

use crate::isobus;

//================================================================================================
/// @struct HardwareInterfaceCanFrame
///
/// @brief A CAN frame for interfacing with a hardware layer, like socket CAN or other interface
//================================================================================================
pub struct HardwareInterfaceCanFrame {
    pub frame: isobus::isobus_HardwareInterfaceCANFrame
}