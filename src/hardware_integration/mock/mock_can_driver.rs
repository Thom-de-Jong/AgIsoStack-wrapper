
use crate::hardware_integration::CanHardwarePluginTrait;
use crate::HardwareInterfaceCanFrame;

pub struct MockCanDriver {
    is_valid: bool
}

impl MockCanDriver {
    pub const fn new() -> Self {
        Self{is_valid: false}
    }
}

impl CanHardwarePluginTrait for MockCanDriver {
    fn is_valid(&mut self) -> bool {
        self.is_valid
    }

    fn close(&mut self) {
        self.is_valid = false;
    }

    fn open(&mut self) {
        self.is_valid = true;
    }

    fn read_frame(&mut self) -> Option<HardwareInterfaceCanFrame> {
        None
    }

    fn write_frame(&mut self, _can_frame: &HardwareInterfaceCanFrame) -> bool {
        true
    }
}
