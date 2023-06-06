//================================================================================================
/// @file name.rs
///
/// @brief A struct that represents a control function's ISONAME
/// @author Thom de Jong
///
/// @copyright 2023 Thom de Jong
//================================================================================================
use crate::isobus;

/// @brief The encoded components that comprise a NAME
#[derive(Copy, Clone)]
pub enum NameParameters {
    Other = -1, //< Undefined "Other"
    IdentityNumber = 0, //< Usually the serial number of the ECU, unique for all similar control functions
    ManufacturerCode = 1, //< The J1939/ISO11783 manufacturer code of the ECU with this NAME
    EcuInstance = 2, //< The ECU instance of the ECU with this NAME. Usually increments in NAME order with similar CFs
    FunctionInstance = 3, //< The function instance of the ECU. Similar to Virtual Terminal number.
    FunctionCode = 4, //< The function of the ECU, as defined by ISO11783
    DeviceClass = 5, //< Also known as the vehicle system from J1939, describes general ECU type
    DeviceClassInstance = 6, //< The instance number of this device class
    IndustryGroup = 7, //< The industry group associated with this ECU, such as "agricultural"
    ArbitraryAddressCapable = 8, //< Defines if this ECU supports address arbitration
}

/// @brief See ISO11783-1 For complete descriptions of the ISO NAME function codes
#[derive(Copy, Clone)]
pub enum Function {
    Engine = 0, //< The typical mechanical power source of the machine
    AuxiliaryPowerUnit = 1, //< Power source for operating systems without the use of the prime drive engine
    ElectricPropulsionControl = 2, //< Control system which operates the drive mechanism when it is electrically powered
    Transmission = 3, //< Mechanical system for altering the speed vs. torque output of the engine
    BatteryPackMonitor = 4, //< Monitors the condition of a battery pack
    ShiftControl = 5, //< Control Unit that determines and transmits onto the network the gear desired by the operator
    PowerTakeOffRearOrPrimary = 6, //< System that controls the mechanical power derived from a prime engine and used to operate auxiliary items
    SteeringAxle = 7, //< Adjusts attack angle of steering axle
    DrivingAxle = 8, //< Adjusts attack angle of driving axle
    SystemControlBrakes = 9, //< Controls service braking system electronically
    SteerAxleControlBrakes = 10, //< Control for actuating the service brakes on a steered axle
    DriveAxleControlBrakes = 11, //< Control for actuating the service brakes on a drive axle
    EnginerRetarder = 12, //< Controller for the retarder capabilities of the engine
    DrivelineRetarder = 13, //< Controller for the retarder capabilities of the driveline
    CruiseControl = 14, //< Control system for maintaining the vehicle's speed at a fixed operator selectable value
    FuelSystem = 15, //< Controls fuel flow from the tank to the filter to the water removal/separator to the engine and then back to the tank
    SteeringControl = 16, //< Controls steering in steer-by-wire
    SteerAxleSuspensionControl = 17, //< Control system for the suspension of a steered axle
    DriveAxleSuspensionControl = 18, //< Control system for the suspension of a driven axle
    InstrumentCluster = 19, //< Gauge display for a vehicle, usually in the cab
    TripRecorder = 20, //< System for accumulating data versus travel of the vehicle
    CabClimateControl = 21, //< System for controlling the climate within the cab of the vehicle
    AerodynamicControl = 22, //< Modifies drag by altering body panels
    VehicleNavigation = 23, //< System associated with the vehicles physical location
    VehicleSecurity = 24, //< System for comparing operator-provided data sequences against reference
    NetworkInterconnectUnit = 25, //< ECU for connecting different network segments together
    BodyControl = 26, //< Can handle suspension control for the body sections independent from the axle sections
    PowerTakeOffFrontOrSecondary = 27, //< System that controls the mechanical power derived from a prime engine and used to operate auxiliary items
    OffVehicleGateway = 28, //< ECU for connecting between vehicle network(s) and an off-vehicle system or network
    VirtualTerminal = 29, //< General-purpose intelligent display with a specific message set defined in ISO 11783-6
    ManagementComputerOne = 30, //< Manages vehicle systems, i.e. powertrain
    PropulsionBatteryCharger = 31, //< Unit used to charge propulsion batteries in an electric vehicle
    HeadwayControl = 32, //< Forward-looking collision avoidance, collision warning, speed controller, or speed control
    SystemMonitor = 33, //< Generic system monitor
    HydraulicPumpControl = 34, //< Pump controller that provides hydraulic power
    SystemControlSuspension = 35, //< Controller responsible for coordinating the over-all suspension of a vehicle
    SystemControlPneumatic = 36, //< Controller responsible for coordinating the pneumatics of a vehicle
    CabController = 37, //< Controller located in/near vehicle cab used to perform functions that are grouped together for convenience
    TirePressureControl = 38, //< Unit that provides control of centralized tire inflation
    IgnitionControl = 39, //< Unit for altering the ignition of an engine
    SeatControl = 40, //< System for controlling the seats (operator and passenger) within the cab
    OperatorControlsLighting = 41, //< Controller for sending the operator lighting controls messages
    WaterPumpControl = 42, //< Controller for a water pump mounted on the vehicle/machine
    TransmissionDisplay = 43, //< Display designed specifically to display transmission information
    ExhaustEmissionControl = 44, //< Emissions controller
    VehicleDynamicStabilityControl = 45, //< Stability controller
    OilSystemMonitor = 46, //< Monitors oil level, life, temperature
    InformationSystemControl = 47, //< Information management for a vehicle's application, such as cargo management
    RampControl = 48, //< Loading unloading chairlift, ramps, lifts or tailgates
    ClutchConverterControl = 49, //< When transmission is distributed this handles torque converter lock-up or engine-transmission connection
    AuxiliaryHeater = 50, //< Primary heat typically being taken from the engine coolant
    ForwardLookingCollisionWarningSystem = 51, //< System which detects and warns of impending collision
    ChassisControl = 52, //< Controls the chassis (not body or cab) components
    AlternatorElectricalChargingSystem = 53, //< Vehicle's primary on-board charging controller
    CommunicationsCellular = 54, //< Radio communications unit designed to communicate via the cellular telephone system
    CommunicationsSatellite = 55, //< Radio communications unit designed specifically to communicate via some satellite system
    CommunicationsRadio = 56, //< Radio unit designed specifically to communicate via a terrestrial p2p system
    OperatorControlsSteeringColumn = 57, //< Unit that gathers the operator inputs from switches/levers/etc and transmits associated messages
    FanDriveControl = 58, //< Primary control system affecting the operation of the main cooling fan
    Starter = 59, //< Mechanical system for initiating rotation in an engine
    CabDisplayCab = 60, //< Used for a fairly elaborate in cab display, non VT and non instrument cluster
    FileServerOrPrinter = 61, //< Printing or file storage unit on the network
    OnboardDiagnosticUnit = 62, //< Tool that can be permanently mounted on the vehicle and which may not support all of the ISO 11783-12 messages
    EngineValveController = 63, //< Control system used to manipulate the actuation of engine intake or exhaust
    EnduranceBraking = 64, //< Sum of all units in a vehicle which enable the driver to brake with virtually no friction
    GasFlowMeasurement = 65, //< Provides measurement of gas flow rates and associated parameters
    IOController = 66, //< Reporting and/or control unit for external input and output channels
    ElectricalSystemController = 67, //< Can include load centres, fuse boxes and power distribution boards
    Reserved = 68, //< Reserved range beginning
    MaxFunctionCode = 127, // Max allocated function code
}

#[derive(Clone, Copy)]
pub struct Name {
    pub name: isobus::isobus_NAME,
}

impl Name {
    pub fn new(raw_name_data: u64) -> Name {
        Name {
            name: isobus::isobus_NAME {
                rawName: raw_name_data,
            },
        }
    }

    pub fn get_arbitrary_address_capable(&self) -> bool {
        unsafe { self.name.get_arbitrary_address_capable() }
    }

    pub fn set_arbitrary_address_capable(&mut self, value: bool) {
        unsafe { self.name.set_arbitrary_address_capable(value); }
    }

    pub fn get_industry_group(&self) -> u8 {
        unsafe { self.name.get_industry_group() }
    }

    pub fn set_industry_group(&mut self, value: u8) {
        unsafe { self.name.set_industry_group(value); }
    }

    pub fn get_device_class_instance(&self) -> u8 {
        unsafe { self.name.get_device_class_instance() }
    }

    pub fn set_device_class_instance(&mut self, value: u8) {
        unsafe { self.name.set_device_class_instance(value); }
    }

    pub fn get_device_class(&self) -> u8 {
        unsafe { self.name.get_device_class() }
    }

    pub fn set_device_class(&mut self, value: u8) {
        unsafe { self.name.set_device_class(value); }
    }

    pub fn get_function_code(&self) -> u8 {
        unsafe { self.name.get_function_code() }
    }

    pub fn set_function_code(&mut self, value: u8) {
        unsafe { self.name.set_function_code(value); }
    }

    pub fn get_function_instance(&self) -> u8 {
        unsafe { self.name.get_function_instance() }
    }

    pub fn set_function_instance(&mut self, value: u8) {
        unsafe { self.name.set_function_instance(value); }
    }

    pub fn get_ecu_instance(&self) -> u8 {
        unsafe { self.name.get_ecu_instance() }
    }

    pub fn set_ecu_instance(&mut self, value: u8) {
        unsafe { self.name.set_ecu_instance(value); }
    }

    pub fn get_manufacturer_code(&self) -> u16 {
        unsafe { self.name.get_manufacturer_code() }
    }

    pub fn set_manufacturer_code(&mut self, value: u16) {
        unsafe { self.name.set_manufacturer_code(value); }
    }

    pub fn get_identity_number(&self) -> u32 {
        unsafe { self.name.get_identity_number() }
    }

    pub fn set_identity_number(&mut self, value: u32) {
        unsafe { self.name.set_identity_number(value); }
    }

    pub fn get_full_name(&self) -> u32 {
        unsafe { self.name.get_identity_number() }
    }

    pub fn set_full_name(&mut self, value: u32) {
        unsafe { self.name.set_identity_number(value); }
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name.rawName == other.name.rawName
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl From<Name> for isobus::isobus_NAME {
    fn from(value: Name) -> Self {
        value.name
    }
}