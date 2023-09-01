import { Address } from "fuels"

// const ADDRESS_BECH32 =
//     "fuel156uyf0hn63k2svsm9n9vl6pjjr9v9lxmwy26v0cpp8tja73eplts34xy4x"

const ADDRESS_BECH32 =
    "fuel1u0kxegh7ayxd26x2fsaf0lh6mzx0culfs6g2lxrd8zapcp6ur45ssfzjll"

const address = new Address(ADDRESS_BECH32)

console.log(address.toB256())
