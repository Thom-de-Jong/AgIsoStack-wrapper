//================================================================================================
/// @file name_filter.rs
///
/// @brief Defines a filter value for an ISONAME component. Used to tell the stack what kind of
/// ECU you want to talk to when creating a partnered control function.
/// @author Thom de Jong
///
/// @copyright 2023 Thom de Jong
//================================================================================================
use crate::isobus;
use crate::name::Name;
use crate::name::NameParameters;

//================================================================================================
/// @struct NameFilter
///
/// @brief A struct that associates a NAME parameter with a value of that parameter.
/// @details This struct is used to match a partner control function with specific criteria that
/// defines it. Use these to define what device you want to talk to.
//================================================================================================
#[derive(Clone, Copy)]
pub struct NameFilter {
    name_filter: isobus::isobus_NAMEFilter,
}

impl NameFilter {
    /// @brief Constructor for the NameFilter
	/// @param[in] name_parameter The component of the NAME to filter on
	/// @param[in] parameter_match_value The value to match with the NameParameter
    pub fn new(name_parameter: NameParameters, parameter_match_value: u32) -> NameFilter {
        NameFilter {
            name_filter: isobus::isobus_NAMEFilter {
                parameter: name_parameter as i32,
                value: parameter_match_value,
            },
        }
    }

    /// @brief Returns the parameter data associated with this filter
	/// @returns The parameter/NAME component associated with this filter
    pub fn get_parameter(&self) -> NameParameters {
        unsafe {
            match self.name_filter.get_parameter() {
                0 => NameParameters::IdentityNumber,
                1 => NameParameters::ManufacturerCode,
                2 => NameParameters::EcuInstance,
                3 => NameParameters::FunctionInstance,
                4 => NameParameters::FunctionCode,
                5 => NameParameters::DeviceClass,
                6 => NameParameters::DeviceClassInstance,
                7 => NameParameters::IndustryGroup,
                8 => NameParameters::ArbitraryAddressCapable,
                _ => NameParameters::Other,
            }
        }
    }

	/// @brief Returns the value associated with this filter
	/// @returns The data associated with this filter component
    pub fn get_value(&self) -> u32 {
        unsafe { self.name_filter.get_value() }
    }

    /// @brief Returns true if a NAME matches this filter class's components
    /// @returns true if a NAME matches this filter class's components
    pub fn check_name_matches_filter(&mut self, name_to_compare: Name) -> bool {
        unsafe { self.name_filter.check_name_matches_filter( &name_to_compare.name) }
    }
}
