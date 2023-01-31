use isobus_plus_plus::{can_hardware_interface, name::NameParameters, Name, NameFilter};

fn main() {
    let name = Name::new(0b1000111100000000111111110000011100000000000111111111111111111111);

    println!("name = {}", name.get_arbitrary_address_capable());
    println!("name = {}", name.get_industry_group());
    println!("name = {}", name.get_device_class_instance());
    println!("name = {}", name.get_device_class());
    println!("name = {}", name.get_function_code());
    println!("name = {}", name.get_function_instance());
    println!("name = {}", name.get_ecu_instance());
    println!("name = {}", name.get_manufacturer_code());
    println!("name = {}", name.get_identity_number());

    let mut filter = NameFilter::new(NameParameters::ArbitraryAddressCapable, 1);

    println!("name = {}", filter.check_name_matches_filter(name));

    // println!("get_number_of_can_channels = {}", can_hardware_interface::get_number_of_can_channels());
}
