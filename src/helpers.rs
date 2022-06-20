use web3::types::Address;

pub fn parse_address(s: &str) -> Address {
    let h = hex::decode(s.trim_start_matches("0x")).expect("could not parse address");
    Address::from_slice(&h)
}

pub fn parse_address_20(s: &str) -> [u8; 20] {
    let h = hex::decode(s.trim_start_matches("0x")).expect("could not parse address");
    let mut a: [u8; 20] = Default::default();
    assert!(h.len() == 20, "address length is incorrect");
    a.copy_from_slice(&h[..20]);
    a
}
