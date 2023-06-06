// #include "isobus/hardware_integration/can_hardware_interface.hpp"
// #include "isobus/hardware_integration/socket_can_interface.hpp"
// #include "isobus/isobus/can_general_parameter_group_numbers.hpp"
// #include "isobus/isobus/can_network_configuration.hpp"
// #include "isobus/isobus/can_network_manager.hpp"
// #include "isobus/isobus/can_partnered_control_function.hpp"
// #include "isobus/isobus/can_transport_protocol.hpp"

// #include <csignal>
// #include <iostream>
// #include <iterator>
// #include <memory>

// #ifdef WIN32
// #include "isobus/hardware_integration/pcan_basic_windows_plugin.hpp"
// static PCANBasicWindowsPlugin canDriver(PCAN_USBBUS1);
// #else
// #include "isobus/hardware_integration/socket_can_interface.hpp"
// static SocketCANInterface canDriver("can0");
// #endif

// static std::shared_ptr<isobus::InternalControlFunction> TestInternalECU = nullptr;
// static isobus::PartneredControlFunction *TestPartner = nullptr;
// std::vector<isobus::NAMEFilter> vtNameFilters;
// const isobus::NAMEFilter testFilter(isobus::NAME::NAMEParameters::FunctionCode, static_cast<std::uint8_t>(isobus::NAME::Function::VirtualTerminal));
// std::uint8_t *TPTestBuffer = nullptr;
// std::uint8_t *ETPTestBuffer = nullptr;
const MAX_TP_SIZE_BYTES: u16 = 1785;
const ETP_TEST_SIZE: u32 = 2048;

// using namespace std;

fn cleanup() {
    // 	CANHardwareInterface::stop();
    // 	if (nullptr != TestPartner)
    // 	{
    // 		delete TestPartner;
    // 		TestPartner = nullptr;
    // 	}
    // 	if (nullptr != TPTestBuffer)
    // 	{
    // 		delete[] TPTestBuffer;
    // 		TPTestBuffer = nullptr;
    // 	}
    // 	if (nullptr != ETPTestBuffer)
    // 	{
    // 		delete[] ETPTestBuffer;
    // 		ETPTestBuffer = nullptr;
    // 	}
}

// void signal_handler(int signum)
// {
// 	cleanup();
// 	exit(signum);
// }

use isobus_plus_plus as isobus;

use isobus::hardware_integration::CanDriver;
use isobus::hardware_integration::CanHardwarePluginTrait;

// use isobus::HardwareInterfaceCanFrame;
use isobus::can_hardware_interface;

// fn update_can_network() {
// 	isobus::CANNetworkManager::CANNetwork.update();
// }

// fn raw_can_glue(raw_frame: &HardwareInterfaceCanFrame) {
// 	isobus::CANNetworkManager::CANNetwork.can_lib_process_rx_message(rawFrame);
// }

// use isobus_plus_plus::can_hardware_interface as CANHardwareInterface;

pub fn setup(can_driver: &mut dyn CanHardwarePluginTrait) {
    can_hardware_interface::set_number_of_can_channels(1);
    can_hardware_interface::assign_can_channel_frame_handler(0, can_driver);

    if (!can_hardware_interface::start()) || (!can_driver.is_valid()) {
        println!("Failed to connect to the socket. The interface might be down.");
    }

    // 	CANHardwareInterface::add_can_lib_update_callback(update_CAN_network, nullptr);
    // 	CANHardwareInterface::add_raw_can_message_rx_callback(raw_can_glue, nullptr);

    // 	std::this_thread::sleep_for(std::chrono::milliseconds(250));

    let test_device_name = isobus::Name::new(0);

    // Make sure you change these for your device!!!!
    // This is an example device that is using a manufacturer code that is currently unused at time of writing
    test_device_name.set_arbitrary_address_capable(true);
    test_device_name.set_industry_group(1);
    test_device_name.set_device_class(0);
    test_device_name.set_function_code(isobus::name::Function::SteeringControl as u8);
    test_device_name.set_identity_number(2);
    test_device_name.set_ecu_instance(0);
    test_device_name.set_function_instance(0);
    test_device_name.set_device_class_instance(0);
    test_device_name.set_manufacturer_code(64);
    // 	vtNameFilters.push_back(testFilter);

    // 	TestInternalECU = std::make_shared<isobus::InternalControlFunction>(TestDeviceNAME, 0x1C, 0);
    // 	TestPartner = new isobus::PartneredControlFunction(0, vtNameFilters);
    // 	std::signal(SIGINT, signal_handler);

    // 	// Wait to make sure address claiming is done. The time is arbitrary.
    // 	std::this_thread::sleep_for(std::chrono::milliseconds(1250));

    // 	// Set up some test CAN messages
    // 	TPTestBuffer = new std::uint8_t[MAX_TP_SIZE_BYTES];
    // 	ETPTestBuffer = new std::uint8_t[ETP_TEST_SIZE];

    // 	for (uint16_t i = 0; i < MAX_TP_SIZE_BYTES; i++)
    // 	{
    // 		TPTestBuffer[i] = (i % 0xFF); // Fill buffer with junk data
    // 	}
    // 	for (uint32_t i = 0; i < ETP_TEST_SIZE; i++)
    // 	{
    // 		ETPTestBuffer[i] = (i % 0xFF); // Fill buffer with junk data
    // 	}
}

pub fn main() {
    let mut can_driver: CanDriver = CanDriver::new();

    setup(&mut can_driver);

    // Send a classic CAN message to a specific destination (8 bytes or less)
    if isobus::CANNetworkManager::CANNetwork.send_can_message(
        0xEF00,
        ETPTestBuffer,
        isobus::CAN_DATA_LENGTH,
        TestInternalECU.get(),
        TestPartner,
    ) {
        println!("Sent a normal CAN Message with length 8");
        std::thread::sleep(std::time::Duration::from_millis(4)); // Arbitrary
    }

    // Send a classic CAN message to global (0xFF) (8 bytes or less)
    if isobus::CANNetworkManager::CANNetwork.send_can_message(
        0xEF00,
        ETPTestBuffer,
        isobus::CAN_DATA_LENGTH,
        TestInternalECU.get(),
    ) {
        println!("Sent a broadcast CAN Message with length 8");
        std::thread::sleep(std::time::Duration::from_millis(4)); // Arbitrary
    }

    // CM Tx Example
    // This loop sends all possible TP CM message sizes.
    // This will take a long time
    // for (std::uint32_t i = 9; i <= MAX_TP_SIZE_BYTES; i++)
    // {
    for i in 0..MAX_TP_SIZE_BYTES {
        // Send message
        if isobus::CANNetworkManager::CANNetwork.send_can_message(
            0xEF00,
            TPTestBuffer,
            i,
            TestInternalECU.get(),
            TestPartner,
        ) {
            println!("Started TP CM Session with length {}", i);
        } else {
            println!("Failed starting TP CM Session with length {}", i);
        }
        // Wait for this session to complete before starting the next
        std::thread::sleep(std::time::Duration::from_millis((i * 2).into())); // Arbitrary
    }

    // BAM Tx Exmaple
    // This loop sends all possible BAM message sizes
    // This will take a very long time
    for i in 0..MAX_TP_SIZE_BYTES {
        // Send message
        if isobus::CANNetworkManager::CANNetwork.send_can_message(
            0xEF00,
            TPTestBuffer,
            i,
            TestInternalECU.get(),
        ) {
            println!("Started BAM Session with length {}", i);
        } else {
            println!("Failed starting BAM Session with length {}", i);
        }
        // Wait for this session to complete before starting the next, or it will fail as only 1 BAM session is possible at a time
        std::thread::sleep(std::time::Duration::from_millis((2 * (isobus::CANNetworkConfiguration::get_minimum_time_between_transport_protocol_bam_frames() * ((i + 1) / 7))).into()));
        // Arbitrary
    }

    // ETP Example
    // Send one ETP message
    if isobus::CANNetworkManager::CANNetwork.send_can_message(
        0xEF00,
        ETPTestBuffer,
        ETP_TEST_SIZE,
        TestInternalECU.get(),
        TestPartner,
    ) {
        println!("Started ETP Session with length {}", ETP_TEST_SIZE);
        std::thread::sleep(std::time::Duration::from_millis(2000)); // Arbitrary
    }

    cleanup();
}
