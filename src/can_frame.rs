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
/// @struct CanFrame
///
/// @brief A CAN frame for interfacing with a hardware layer, like socket CAN or other interface
//================================================================================================
pub struct CanFrame {
    pub frame: isobus::isobus_HardwareInterfaceCANFrame
}

impl embedded_can::Frame for CanFrame {
    fn new(id: impl Into<embedded_can::Id>, data: &[u8]) -> Option<Self> {
        let id: u32 = match id.into() {
            embedded_can::Id::Extended(id) => {id.as_raw()},
            embedded_can::Id::Standard(id) => {id.as_raw() as u32},
        };

        let dlc = usize::min(data.len(), 8);
        let mut temp_data: Vec<u8> = Vec::new();

        if let Some(d) = data.get(0..dlc) {
            temp_data.extend_from_slice(d);
        }

        Some(Self {
            frame: isobus::isobus_HardwareInterfaceCANFrame {
                timestamp_us: crate::utility::system_timing::get_time_elapsed().as_micros() as u64,
                identifier: id,
                channel: todo!(),
                data: temp_data,
                dataLength: todo!(),
                isExtendedFrame: todo!()
            },
        })
    }

    fn new_remote(id: impl Into<embedded_can::Id>, dlc: usize) -> Option<Self> {
        None
    }

    fn is_extended(&self) -> bool {
        self.frame.isExtendedFrame
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> embedded_can::Id {
        match self.is_extended() {
            true => embedded_can::Id::Extended(embedded_can::ExtendedId::new(self.frame.identifier).unwrap_or(embedded_can::ExtendedId::MAX)),
            false => embedded_can::Id::Standard(embedded_can::StandardId::new(self.frame.identifier as u16).unwrap_or(embedded_can::StandardId::MAX)),
        }
    }

    fn dlc(&self) -> usize {
        self.frame.dataLength as usize
    }

    fn data(&self) -> &[u8] {
        self.frame.data.as_slice()
    }
}