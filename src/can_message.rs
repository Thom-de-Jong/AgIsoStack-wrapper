//================================================================================================
/// @file can_message.rs
///
/// @brief An abstraction  of a CAN message, could be > 8 data bytes.
/// @author Thom de Jong
///
/// @copyright 2023 Thom de Jong
//================================================================================================

use crate::{isobus, can_control_function::ControlFunction, can_identifier::CANIdentifier};

// #include "isobus/isobus/can_control_function.hpp"
// #include "isobus/isobus/can_identifier.hpp"

// #include <vector>

/// @brief The internal message type
pub enum Type {
    Transmit, //< Message is to be transmitted from the stack
    Receive, //< Message is being received
    Internal, //< Message is being used internally as data storage for a protocol
}

/// @brief The different byte formats that can be used when reading bytes from the buffer.
pub enum ByteFormat {
    LittleEndian,
    BigEndian,
}


//================================================================================================
/// @class CANMessage
///
/// @brief A class that represents a generic CAN message of arbitrary length.
//================================================================================================
pub struct CanMessage {
    can_message: isobus::isobus_CANMessage
}


impl CanMessage {
	/// @brief Constructor for a CAN message
	/// @param[in] CANPort The can channel index the message uses
    pub fn new(can_port: u8) -> Self {
        Self {
            can_message: unsafe { isobus::isobus_CANMessage::new(can_port) },
        }
    }

	/// @brief Returns the CAN message type
	/// @returns The type of the CAN message
	pub fn message_type(&self) -> Type {
        unsafe {
            match self.can_message.get_type() {
                isobus::isobus_CANMessage_Type_Internal => Type::Internal,
                isobus::isobus_CANMessage_Type_Receive => Type::Receive,
                isobus::isobus_CANMessage_Type_Transmit => Type::Transmit,
                _ => Type::Internal,
            }
        }
    }

	/// @brief Gets a reference to the data in the CAN message
	/// @returns A reference to the data in the CAN message
	pub fn data(&mut self) -> &[u8] {
        unsafe {
            &*self.can_message.get_data()
        }
    }

	/// @brief Returns the length of the data in the CAN message
	/// @returns The message data payload length
    pub fn data_length(&mut self) -> usize {
        self.data().len()
    }

	/// @brief Gets the source control function that the message is from
	/// @returns The source control function that the message is from
	pub fn source_control_function(&self) -> ControlFunction {
        // unsafe {
        //     (*self.can_message.get_source_control_function()).into()
        // }
        todo!()
    }

	/// @brief Gets the destination control function that the message is to
	/// @returns The destination control function that the message is to
    pub fn destination_control_function(&self) -> ControlFunction {
        // unsafe {
        //     (*self.can_message.get_destination_control_function()).into()
        // }
        todo!()
    }

	/// @brief Returns the identifier of the message
	/// @returns The identifier of the message
    pub fn identifier(&self) -> CANIdentifier {
        unsafe {
            self.can_message.get_identifier().into()
        }
    }

	/// @brief Returns the unique message ID
	/// @returns The unique message ID
    pub fn message_unique_id(&self) -> u32 {
        unsafe {
            self.can_message.get_message_unique_id()
        }
    }

	/// @brief Returns the CAN channel index associated with the message
	/// @returns The CAN channel index associated with the message
    pub fn get_can_port_index(&mut self) -> u8 {
        todo!()
    }

	// /// @brief Get a 8-bit unsigned byte from the buffer at a specific index.
	// /// A 8-bit unsigned byte can hold a value between 0 and 255.
	// /// @details This function will return the byte at the specified index in the buffer.
	// /// @param[in] index The index to get the byte from
	// /// @return The 8-bit unsigned byte
	// // std::uint8_t get_uint8_at(const std::size_t index);
    // pub fn get_can_port_index() -> u8 {
    //     todo!()
    // }


// 	/// @brief Get a 16-bit unsigned integer from the buffer at a specific index.
// 	/// A 16-bit unsigned integer can hold a value between 0 and 65535.
// 	/// @details This function will return the 2 bytes at the specified index in the buffer.
// 	/// @param[in] index The index to get the 16-bit unsigned integer from
// 	/// @param[in] format The byte format to use when reading the integer
// 	/// @return The 16-bit unsigned integer
// 	std::uint16_t get_uint16_at(const std::size_t index, const ByteFormat format = ByteFormat::LittleEndian);
// 	/// @brief Get a right-aligned 24-bit integer from the buffer (returned as a uint32_t) at a specific index.
// 	/// A 24-bit number can hold a value between 0 and 16,777,215.
// 	/// @details This function will return the 3 bytes at the specified index in the buffer.
// 	/// @param[in] index The index to get the 24-bit unsigned integer from
// 	/// @param[in] format The byte format to use when reading the integer
// 	/// @return The 24-bit unsigned integer, right aligned into a uint32_t
// 	std::uint32_t get_uint24_at(const std::size_t index, const ByteFormat format = ByteFormat::LittleEndian);
// 	/// @brief Get a 32-bit unsigned integer from the buffer at a specific index.
// 	/// A 32-bit unsigned integer can hold a value between 0 and 4294967295.
// 	/// @details This function will return the 4 bytes at the specified index in the buffer.
// 	/// @param[in] index The index to get the 32-bit unsigned integer from
// 	/// @param[in] format The byte format to use when reading the integer
// 	/// @return The 32-bit unsigned integer
// 	std::uint32_t get_uint32_at(const std::size_t index, const ByteFormat format = ByteFormat::LittleEndian);
// 	/// @brief Get a 64-bit unsigned integer from the buffer at a specific index.
// 	/// A 64-bit unsigned integer can hold a value between 0 and 18446744073709551615.
// 	/// @details This function will return the 8 bytes at the specified index in the buffer.
// 	/// @param[in] index The index to get the 64-bit unsigned integer from
// 	/// @param[in] format The byte format to use when reading the integer
// 	/// @return The 64-bit unsigned integer
// 	std::uint64_t get_uint64_at(const std::size_t index, const ByteFormat format = ByteFormat::LittleEndian);
// 	/// @brief Get a bit-boolean from the buffer at a specific index.
// 	/// @details This function will return whether the bit(s) at the specified index in the buffer is/are (all) equal to 1.
// 	/// @param[in] byteIndex The byte index to start reading the boolean from
// 	/// @param[in] bitIndex The bit index to start reading the boolean from, ranging from 0 to 7
// 	/// @param[in] length The number of bits to read, maximum of (8 - bitIndex)
// 	/// @return True if (all) the bit(s) are set, false otherwise
// 	bool get_bool_at(const std::size_t byteIndex, const std::uint8_t bitIndex, const std::uint8_t length = 1);
// 	/// @brief ISO11783-3 defines this: The maximum number of packets that can be sent in a single connection
// 	/// with extended transport protocol is restricted by the extended data packet offset (3 bytes).
// 	/// This yields a maximum message size of (2^24-1 packets) x (7 bytes/packet) = 117440505 bytes
// 	/// @returns The maximum length of any CAN message as defined by ETP in ISO11783
// 	static const std::uint32_t ABSOLUTE_MAX_MESSAGE_LENGTH = 117440505;
// protected:
// 	std::vector<std::uint8_t> data; ///< A data buffer for the message, used when not using data chunk callbacks
// 	ControlFunction *source; ///< The source control function of the message
// 	ControlFunction *destination; ///< The destination control function of the message
// 	CANIdentifier identifier; ///< The CAN ID of the message
// 	Type messageType; ///< The internal message type associated with the message
// 	const std::uint32_t messageUniqueID; ///< The unique ID of the message, an internal value for tracking and stats
// 	const std::uint8_t CANPortIndex; ///< The CAN channel index associated with the message
// private:
// 	static std::uint32_t lastGeneratedUniqueID; ///< A unique, sequential ID for this CAN message
}

impl From<CanMessage> for isobus::isobus_CANMessage {
    fn from(_value: CanMessage) -> Self {
        todo!()
    }
}