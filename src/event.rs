elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait EventModule {
    #[event("createOrder")]
    fn create_order_event(
        &self,
        #[indexed] order_id: usize,
        #[indexed] owner: ManagedAddress,
        #[indexed] offered_token: TokenIdentifier,
        #[indexed] offered_amount: BigUint,
        #[indexed] offered_token_decimals: u32,
        #[indexed] wanted_token: TokenIdentifier,
        #[indexed] wanted_amount: BigUint,
        #[indexed] wanted_token_decimals: u32,
        #[indexed] creation_timestamp: u64,
    );

    #[event("cancelOrder")]
    fn cancel_order_event(
        &self,
        #[indexed] order_id: usize,
        #[indexed] user_address: ManagedAddress,
        #[indexed] timestamp: u64,
    );

    #[event("acceptOrder")]
    fn accept_order_event(
        &self,
        #[indexed] order_id: usize,
        #[indexed] offerer: ManagedAddress,
        #[indexed] acceptor: ManagedAddress,
        #[indexed] percentge: u64,
        #[indexed] timestamp: u64,
    );
}