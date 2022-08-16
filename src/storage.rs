elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const TOTAL_PERCENTAGE: u64 = 1000;    // 1000 = 100%

#[elrond_wasm::module]
pub trait StorageModule
{
    //
    #[view(getTreasuryAddress)]
    #[storage_mapper("treasury_address")]
    fn treasury_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[only_owner]
    #[endpoint(setTreasuryAddress)]
    fn set_treasury_address(&self, treasury_address: ManagedAddress) {
        self.treasury_address().set(treasury_address);
    }

    //
    #[view(getFee)]
    #[storage_mapper("fee")]
    fn fee(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setFee)]
    fn set_fee(&self, fee: u64) {
        require!(
            fee <= TOTAL_PERCENTAGE,
            "fee cannot be greater than 100%"
        );
        self.fee().set(fee);
    }

    //
    #[view(getExpirationPeriod)]
    #[storage_mapper("expiration_period")]
    fn expiration_period(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setExpirationPeriod)]
    fn set_expiration_period(&self, expiration_period: u64) {
        self.expiration_period().set(expiration_period);
    }

    //
    #[view(getPaused)]
    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<bool>;
    
    #[only_owner]
    #[endpoint(pause)]
    fn pause(&self) {
        self.paused().set(true);
    }

    #[only_owner]
    #[endpoint(unpause)]
    fn unpause(&self) {
        self.paused().set(false);
    }

    ///////////////////////////////////////////////////////////////////////////////////
    #[view(getLastOrderId)]
    #[storage_mapper("last_order_id")]
    fn last_order_id(&self) -> SingleValueMapper<usize>;

    #[view(getLiveOrderIds)]
    #[storage_mapper("live_order_ids")]
    fn live_order_ids(&self) -> UnorderedSetMapper<usize>;

    #[view(getOrderOwner)]
    #[storage_mapper("order_owner")]
    fn order_owner(&self, offer_id: usize) -> SingleValueMapper<ManagedAddress>;

    #[view(getOfferedToken)]
    #[storage_mapper("offered_token")]
    fn offered_token(&self, offer_id: usize) -> SingleValueMapper<TokenIdentifier>;

    #[view(getOfferedAmount)]
    #[storage_mapper("offered_amount")]
    fn offered_amount(&self, offer_id: usize) -> SingleValueMapper<BigUint>;

    #[view(getOfferedTokenDecimals)]
    #[storage_mapper("offered_token_decimals")]
    fn offered_token_decimals(&self, offer_id: usize) -> SingleValueMapper<u32>;

    #[view(getWantedToken)]
    #[storage_mapper("wanted_token")]
    fn wanted_token(&self, offer_id: usize) -> SingleValueMapper<TokenIdentifier>;

    #[view(getWantedAmount)]
    #[storage_mapper("wanted_amount")]
    fn wanted_amount(&self, offer_id: usize) -> SingleValueMapper<BigUint>;

    #[view(getWantedTokenDecimals)]
    #[storage_mapper("wanted_token_decimals")]
    fn wanted_token_decimals(&self, offer_id: usize) -> SingleValueMapper<u32>;


    #[view(getCreationTimestamp)]
    #[storage_mapper("creation_timestamp")]
    fn creation_timestamp(&self, offer_id: usize) -> SingleValueMapper<u64>;
}