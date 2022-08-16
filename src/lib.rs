#![no_std]
#![feature(generic_associated_types)]
#![feature(let_chains)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod event;
mod logic;
mod state;
mod storage;

use state::{HandshakeSetting, Order};

#[elrond_wasm::derive::contract]
pub trait Handshake:
    logic::LogicModule
    + storage::StorageModule
    + event::EventModule
{
    #[init]
    fn init(
        &self,
        treasury_address: ManagedAddress,
        fee: u64,
        expiration_period: u64,
    ) {
        self.treasury_address().set(treasury_address);
        self.set_fee(fee);
        self.expiration_period().set(expiration_period);
    }

    #[view(viewHandshakeSetting)]
    fn view_handshake_setting(&self) -> HandshakeSetting<Self::Api> {
        HandshakeSetting {
            treasury_address: self.treasury_address().get(),
            fee: self.fee().get(),
            paused: self.paused().get(),
            number_of_orders: self.live_order_ids().len(),
        }
    }

    #[view(viewOrder)]
    fn view_order(&self, order_id: usize) -> Order<Self::Api> {
        require!(
            self.live_order_ids().contains(&order_id),
            "invalid order_id"
        );

        Order {
            order_id,
            owner: self.order_owner(order_id).get(),
            offered_token: self.offered_token(order_id).get(),
            offered_amount: self.offered_amount(order_id).get(),
            offered_token_decimals: self.offered_token_decimals(order_id).get(),
            wanted_token: self.wanted_token(order_id).get(),
            wanted_amount: self.wanted_amount(order_id).get(),
            wanted_token_decimals: self.wanted_token_decimals(order_id).get(),
            creation_timestamp: self.creation_timestamp(order_id).get(),
        }
    }

    #[view(viewOrders)]
    fn view_orders(&self) -> MultiValueEncoded<Order<Self::Api>> {
        let mut items = MultiValueEncoded::new();

        let live_order_ids = self.live_order_ids();
        for order_id in live_order_ids.iter() {
            items.push(self.view_order(order_id));
        }

        items
    }
}
