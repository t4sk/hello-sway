contract;

abi LiquidityBook {
    #[storage(read, write)]
    fn init() {}

    #[storage(read, write)]
    fn mint();

    #[storage(read, write)]
    fn burn();

    #[storage(read, write)]
    fn swap() -> u64;
}

abi LiquidityBookInfo {

}

impl LiquidityBook for Contract {
    #[storage(read, write)]
    fn init() {}

    #[storage(read, write)]
    fn mint() {}

    #[storage(read, write)]
    fn burn() {}

    #[storage(read, write)]
    fn swap() -> u64 {
        0
    }
}

impl LiquidityBookInfo for Contract {

}
