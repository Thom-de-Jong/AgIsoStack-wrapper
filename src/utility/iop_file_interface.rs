//================================================================================================
/// @file iop_file_interface.rs
///
/// @brief Implementation of class that manages reading IOP files
/// @author Thom de Jong
///
/// @copyright 2023 Thom de Jong
//================================================================================================

pub fn read_iop_file(file_name: &str) -> Vec<u8> {
	std::fs::read(file_name).unwrap_or_else(|_| { Vec::new() })
}

pub fn hash_object_pool_to_version(iop_data: &[u8]) -> String {
    let mut seed: usize = iop_data.len();
	for x in iop_data.to_owned() {
		let mut x: usize = x.clone().into();
		x = ((x >> 16) ^ x) * 0x45d9f3b;
		x = ((x >> 16) ^ x) * 0x45d9f3b;
		x = (x >> 16) ^ x;
		seed ^= x + 0x9e3779b9 + (seed << 6) + (seed >> 2);
	}
	format!("{:X}", seed)
}