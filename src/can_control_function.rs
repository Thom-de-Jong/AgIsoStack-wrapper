//================================================================================================
/// @file can_control_function.rs
///
/// @brief Defines a base class to represent a generic ISOBUS control function.
/// @author Thom de Jong
///
/// @copyright 2023 Thom de Jong
//================================================================================================

use crate::isobus;
use crate::Name;

/// @brief The type of the control function
pub enum Type {
    Internal, //< The control function is part of our stack and can address claim
	External, //< The control function is some other device on the bus
	Partnered, //< An external control function that you explicitly want to talk to
}

//================================================================================================
/// @class ControlFunction
///
/// @brief A class that describes an ISO11783 control function, which includes a NAME and address.
//================================================================================================
pub struct ControlFunction {
	control_function: isobus::isobus_ControlFunction
}

impl ControlFunction {
	/// @brief The base class constructor for a control function
	/// @param[in] NAMEValue The NAME of the control function
	/// @param[in] addressValue The current address of the control function
	/// @param[in] CANPort The CAN channel index that the control function communicates on
	pub fn new(name: Name, address: u8, can_port: u8) -> Self {
		Self {
            control_function: unsafe { isobus::isobus_ControlFunction::new(name.into(), address, can_port) },
        }
	}

	/// @brief The base class destructor for a control function
	// virtual ~ControlFunction();

	/// @brief Returns the current address of the control function
	/// @returns The current address of the control function
	pub fn address(&self) -> u8 {
		unsafe {
			self.control_function.get_address()
		}
	}

	/// @brief Describes if the control function has a valid address (not NULL or global)
	/// @returns true if the address is < 0xFE
	pub fn is_address_valid(&self) -> bool {
		unsafe {
			self.control_function.get_address_valid()
		}
	}

	/// @brief Returns the CAN channel index the control function communicates on
	/// @returns The control function's CAN channel index
	pub fn can_port(&self) -> u8 {
		unsafe {
			self.control_function.get_can_port()
		}
	}

	/// @brief Returns the NAME of the control function as described by its address claim message
	/// @returns The control function's NAME
	pub fn name(&self) -> Name {
		unsafe {
			Name::new(self.control_function.get_NAME().rawName)
		}
	}

	/// @brief Returns the `Type` of the control function
	/// @returns The control function type
	pub fn get_type(&self) -> Type {
		unsafe {
            match self.control_function.get_type() {
                isobus::isobus_ControlFunction_Type_Internal => Type::Internal,
                isobus::isobus_ControlFunction_Type_External => Type::External,
                isobus::isobus_ControlFunction_Type_Partnered => Type::Partnered,
                _ => Type::Internal,
            }
        }
	}
}

impl From<ControlFunction> for isobus::isobus_ControlFunction {
    fn from(value: ControlFunction) -> Self {
        value.control_function
    }
}

impl From<isobus::isobus_ControlFunction> for ControlFunction {
    fn from(value: isobus::isobus_ControlFunction) -> Self {
        Self { control_function: value, }
	}
}