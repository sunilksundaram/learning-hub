use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>; // include a lifetime
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_address().unwrap();
    converted_address
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn my_test() {
        dbg!("Hello");
    }

    #[test]
    fn test_get_ethereum_data() {
        let addr = Address::from_str("0xd31DCEe46D63D510bF4B6571d375D093A9fdC1eB").unwrap();
        assert_eq!(
            addr,
            Address::from_str("0xd31DCEe46D63D510bF4B6571d375D093A9fdC1eB").unwrap()
        );
    }

    #[test]
    fn test_get_ethereum_data2() {
        let addr = Address::from_str("0xd31DCEe46D63D510bF4B6571d375D093A9fdC1eB").unwrap();
        let new_addr = get_ethereum_data(addr);
        assert_eq!(
            new_addr,
            Address::from_str("0xd31DCEe46D63D510bF4B6571d375D093A9fdC1eB").unwrap()
        );
    }

    #[test]
    fn test_get_ethereum_data3() {
        let new_addr = get_ethereum_data("0xd31DCEe46D63D510bF4B6571d375D093A9fdC1eB");
        assert_eq!(
            new_addr,
            Address::from_str("0xd31DCEe46D63D510bF4B6571d375D093A9fdC1eB").unwrap()
        );
    }
}
