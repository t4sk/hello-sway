script;

// Address
// Contract Id
// Identity
fn main() {
    // Address
    let b: b256 = 0x000000000000000000000000000000000000000000000000000000000000002A;
    let addr: Address = Address::from(b);
    let b: b256 = addr.into();

    // Contract id
    let b: b256 = 0x000000000000000000000000000000000000000000000000000000000000002A;
    let my_contract_id: ContractId = ContractId::from(b);
    let b: b256 = my_contract_id.into();

    // Identity type
    let raw_addr: b256 = 0xddec0e7e6a9a4a4e3e57d08d080d71a299c628a46bc609aab4627695679421ca;
    let addr = Address::from(raw_addr);
    let my_identity: Identity = Identity::Address(addr);

    // Match on identity
    let my_id: Address = match my_identity {
        Identity::Address(id) => id,
        Identity::ContractId(id) => revert(0),
    };
}