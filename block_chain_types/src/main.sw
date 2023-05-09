script;

// Address
// Contract Id
fn main() {
    // Address
    let my_number: b256 = 0x000000000000000000000000000000000000000000000000000000000000002A;
    let my_address: Address = Address::from(my_number);
    let forty_two: b256 = my_address.into();

    // Contract id
    let my_number: b256 = 0x000000000000000000000000000000000000000000000000000000000000002A;
    let my_contract_id: ContractId = ContractId::from(my_number);
    let forty_two: b256 = my_contract_id.into();

    // Identity type
    let raw_address: b256 = 0xddec0e7e6a9a4a4e3e57d08d080d71a299c628a46bc609aab4627695679421ca;
    let my_identity: Identity = Identity::Address(Address::from(raw_address));


    // Match on identity
    let my_id: Address = match my_identity {
        Identity::Address(id) => id,
        Identity::ContractId(id) => revert(0),
    };
}