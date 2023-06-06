//================================================================================================
/// @file can_identifier.rs
///
/// @brief A representation of a classical CAN identifier with utility functions for ectracting
/// values that are encoded inside, along with some helpful constants.
/// @author Thom de Jong
///
/// @copyright 2023 Thom de Jong
//================================================================================================

use crate::isobus;

/// @brief Defines all the CAN frame priorities that can be encoded in a frame ID
pub enum CANPriority {
    PriorityHighest0 = 0, //< Highest CAN priority
    Priority1 = 1, //< Priority highest - 1
    Priority2 = 2, //< Priority highest - 2
    Priority3 = 3, //< Priority highest - 3 (Control messages priority)
    Priority4 = 4, //< Priority highest - 4
    Priority5 = 5, //< Priority highest - 5
    PriorityDefault6 = 6, //< The default priority
    PriorityLowest7 = 7, //< The lowest priority
}

/// @brief Defines if a frame is a standard (11 bit) or extended (29 bit) ID frame
pub enum Type {
    Standard = 0, //< Frame is an 11bit ID standard (legacy) message with no PGN and highest priority
    Extended = 1, //< Frame is a modern 29 bit ID CAN frame
}

//================================================================================================
/// @class CANIdentifier
///
/// @brief A utility class that allows easy interpretation of a 32 bit CAN identifier
//================================================================================================
pub struct CANIdentifier {
	identifier: isobus::isobus_CANIdentifier
}

impl CANIdentifier {
    /// @brief Constructor for a CAN Identifier class based on a raw 32 bit ID
	/// @param[in] rawIdentifierData The raw 32 bit ID to interpret
    pub fn new(raw_identifier_data: u32) -> Self {
        Self {
            identifier: unsafe { isobus::isobus_CANIdentifier::new(raw_identifier_data) },
        }
    }

    /// @brief Returns the raw encoded ID of the CAN identifier
	/// @returns The raw encoded ID of the CAN identifier
	pub fn raw_identifier(&self) -> u32 {
        unsafe {
            self.identifier.get_identifier()
        }
    }

	/// @brief Returns the identifier type (standard vs extended)
	/// @returns The identifier type (standard vs extended)
    pub fn identifier_type(&self) -> Type {
        unsafe {
            match self.identifier.get_priority() {
                isobus::isobus_CANIdentifier_Type_Standard => Type::Standard,
                isobus::isobus_CANIdentifier_Type_Extended =>  Type::Extended,
                _ =>  Type::Standard,
            }
        }
    }

	/// @brief Returns the PGN encoded in the identifier
	/// @returns The PGN encoded in the identifier
    pub fn parameter_group_number(&self) -> u32 {
        unsafe {
            self.identifier.get_parameter_group_number()
        }
    }

	/// @brief Returns the priority of the frame encoded in the identifier
	/// @returns The priority of the frame encoded in the identifier
    pub fn priority(&self) -> CANPriority {
        unsafe {
            match self.identifier.get_priority() {
                isobus::isobus_CANIdentifier_CANPriority_PriorityHighest0 => CANPriority::PriorityHighest0,
                isobus::isobus_CANIdentifier_CANPriority_Priority1 => CANPriority::Priority1,
                isobus::isobus_CANIdentifier_CANPriority_Priority2 => CANPriority::Priority2,
                isobus::isobus_CANIdentifier_CANPriority_Priority3 => CANPriority::Priority3,
                isobus::isobus_CANIdentifier_CANPriority_Priority4 => CANPriority::Priority4,
                isobus::isobus_CANIdentifier_CANPriority_Priority5 => CANPriority::Priority5,
                isobus::isobus_CANIdentifier_CANPriority_PriorityDefault6 => CANPriority::PriorityDefault6,
                isobus::isobus_CANIdentifier_CANPriority_PriorityLowest7 => CANPriority::PriorityLowest7,
                _ => CANPriority::PriorityLowest7,
            }
        }
    }

	/// @brief Returns the destination address of the frame encoded in the identifier
	/// @returns The destination address of the frame encoded in the identifier
    pub fn destination_address(&self) -> u8 {
        unsafe {
            self.identifier.get_destination_address()
        }
    }

	/// @brief Returns the source address of the frame encoded in the identifier
	/// @returns The source address of the frame encoded in the identifier
    pub fn source_address(&self) -> u8 {
        unsafe {
            self.identifier.get_source_address()
        }
    }

	/// @brief Returns if the ID is valid based on some range checking
	/// @returns Frame valid status
    pub fn is_valid(&self) -> bool {
        unsafe {
            self.identifier.get_is_valid()
        }
    }

}


impl From<CANIdentifier> for isobus::isobus_CANIdentifier {
    fn from(value: CANIdentifier) -> Self {
        value.identifier
    }
}

impl From<isobus::isobus_CANIdentifier> for CANIdentifier {
    fn from(value: isobus::isobus_CANIdentifier) -> Self {
        Self { identifier: value, }
	}
}

		
// 		explicit CANIdentifier(std::uint32_t rawIdentifierData);

// 		/// @brief Constructor for a CAN Identifier class based on all discrete components
// 		/// @param[in] identifierType Type of frame, either standard 11 bit ID, or extended 29 bit ID
// 		/// @param[in] pgn The parameter group number encoded in the frame (extended only)
// 		/// @param[in] priority The priority of the frame (extended only)
// 		/// @param[in] destinationAddress The destination address of the frame
// 		/// @param[in] sourceAddress The source address of the frame
// 		CANIdentifier(Type identifierType,
// 		              std::uint32_t pgn,
// 		              CANPriority priority,
// 		              std::uint8_t destinationAddress,
// 		              std::uint8_t sourceAddress);

// 		/// @brief Copy constructor for a CAN Identifier
// 		/// @param[in] copiedObject The object to copy
// 		CANIdentifier(const CANIdentifier &copiedObject);

// 		/// @brief Destructor for the CANIdentifier
// 		~CANIdentifier();

// 		/// @brief Assignment operator for a CAN identifier
// 		/// @param[in] obj rhs of the operator
// 		CANIdentifier &operator=(const CANIdentifier &obj);

		

// 		static constexpr std::uint32_t IDENTIFIER_TYPE_BIT_MASK = 0x80000000; ///< This bit denotes if the frame is standard or extended format
// 		static constexpr std::uint32_t UNDEFINED_PARAMETER_GROUP_NUMBER = 0xFFFFFFFF; ///< A fake PGN used internally to denote a NULL PGN
// 		static constexpr std::uint8_t GLOBAL_ADDRESS = 0xFF; ///< The broadcast CAN address
// 		static constexpr std::uint8_t NULL_ADDRESS = 0xFE; ///< The NULL CAN address as defined by ISO11783

// 	private:
// 		static constexpr std::uint32_t BROADCAST_PGN_MASK = 0x0003FFFF; ///< Broadcast PGNs don't mask off the bits used for destination in the PGN
// 		static constexpr std::uint32_t DESTINATION_SPECIFIC_PGN_MASK = 0x0003FF00; ///< Destination specific PGNs mask the destination out of the PGN itself
// 		static constexpr std::uint32_t PDU2_FORMAT_MASK = 0x00F00000; ///< Mask that denotes the ID as being PDU2 format
// 		static constexpr std::uint8_t PARAMTER_GROUP_NUMBER_OFFSET = 8; ///< PGN is offset 8 bits into the ID
// 		static constexpr std::uint8_t PRIORITY_DATA_BIT_OFFSET = 26; ///< Priority is offset 26 bits into the ID

// 		std::uint32_t m_RawIdentifier; ///< The raw encoded 29 bit ID
// 	};

// } // namespace isobus

// #endif // CAN_IDENTIFIER_HPP