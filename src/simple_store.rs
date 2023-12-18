use revm::primitives::{Address, U256};
use rustry_macros::{huff, rustry_test};
use rustry_test::{
    prelude::*,
    utils::abi::{abi_decode, abi_encode_signature, AbiType},
};
#[allow(dead_code)]
fn set_up() {
    let mut provider = Provider::default();

    let simple_store = huff! {"./src/contracts/simple_store.huff"};

    let _simple_store = simple_store.deploy(&mut provider);
}

#[rustry_test(set_up)]
fn test_deployment() {
    assert_ne!(simple_store.address, Address::ZERO);
}
#[rustry_test(set_up)]
fn test_set_value() {
    let number = 2;
    let value = Uint::<256, 4>::from(*&number);

    let data = abi_encode_signature("setValue(uint256)", vec![value.to_be_bytes::<32>().to_vec()]);

    provider.call(simple_store.address, data.into()).success();

    let number = get_value(simple_store.address, &mut provider);

    assert_eq!(number, U256::from(number));
}
#[allow(dead_code)]
fn get_value(caddr: Address, provider: &mut Provider) -> U256 {
    let ret = provider.staticcall(caddr, abi_encode_signature("getValue()", vec![]).into());
    assert!(ret.is_success());
    let data = ret.get_data();
    U256::from_be_bytes::<32>(abi_decode(data, vec![AbiType::Uint]).try_into().unwrap())
}

fn main() {}
