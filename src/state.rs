elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct Order<M: ManagedTypeApi> {
    pub order_id: usize,
    pub owner: ManagedAddress<M>,
    pub offered_token: TokenIdentifier<M>,
    pub offered_amount: BigUint<M>,
    pub offered_token_decimals: u32,
    pub wanted_token: TokenIdentifier<M>,
    pub wanted_amount: BigUint<M>,
    pub wanted_token_decimals: u32,
    pub creation_timestamp: u64,
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct HandshakeSetting<M: ManagedTypeApi> {
    pub treasury_address: ManagedAddress<M>,
    pub fee: u64,
    pub paused: bool,
    pub number_of_orders: usize,
}