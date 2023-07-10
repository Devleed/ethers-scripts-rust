use std::{collections::HashMap, fs};

use ethers::types::{Address, H160};

use crate::{account, utils};

pub fn add_beneficiary() {
    let account_name = account::get_account_name().unwrap();

    let mut account_path = String::from("accounts/");
    account_path.push_str(&account_name);
    account_path.push_str("/beneficiaries.json");

    let beneficiaries_json = fs::read_to_string(&account_path).unwrap_or_else(|_e| {
        fs::write(&account_path, "{}".as_bytes()).unwrap();

        fs::read_to_string(&account_path).unwrap()
    });

    let mut beneficiaries: HashMap<String, Address> =
        serde_json::from_str(beneficiaries_json.trim()).unwrap();

    let mut beneficiary_name = String::new();
    utils::take_user_input(
        "Beneficiary name",
        &mut beneficiary_name,
        "Enter beneficiary name",
    );

    let mut beneficiary_address_str = String::new();
    utils::take_user_input(
        "Beneficiary address",
        &mut beneficiary_address_str,
        "Enter beneficiary address",
    );

    let beneficiary_address: H160 = beneficiary_address_str.trim().parse().unwrap();

    beneficiaries.insert(String::from(beneficiary_name.trim()), beneficiary_address);

    let new_beneficiaries_str = serde_json::to_string(&beneficiaries).unwrap();
    fs::write(&account_path, new_beneficiaries_str.as_bytes()).unwrap();

    println!("Beneficiary added successfully");
}
