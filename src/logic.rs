elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::storage::{TOTAL_PERCENTAGE};

#[elrond_wasm::module]
pub trait LogicModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{
    #[payable("*")]
    #[endpoint(createOrder)]
    fn create_order(
        &self,
        offered_token: TokenIdentifier,
        offered_amount: BigUint,
        offered_token_decimals: u32,
        wanted_token: TokenIdentifier,
        wanted_amount: BigUint,
        wanted_token_decimals: u32,
    ) {
        // check if SC is paused
        self.require_activation();

        let payments = self.call_value().all_esdt_transfers();
        require!(
            payments.len() == 1,
            "wrong number of payment tokens"
        );

        let payment_token = payments.get(0).token_identifier;
        let payment_amount = payments.get(0).amount;

        require!(
            payment_token == offered_token,
            "offered_token not match with paid token"
        );
        require!(
            payment_amount == offered_amount,
            "offered_amount not match with paid amount"
        );

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let order_id = self.last_order_id().get() + 1;
        self.last_order_id().set(order_id);
        self.live_order_ids().insert(order_id);

        self.order_owner(order_id).set(&caller);
        self.offered_token(order_id).set(&offered_token);
        self.offered_amount(order_id).set(&offered_amount);
        self.offered_token_decimals(order_id).set(&offered_token_decimals);
        self.wanted_token(order_id).set(&wanted_token);
        self.wanted_amount(order_id).set(&wanted_amount);
        self.wanted_token_decimals(order_id).set(&wanted_token_decimals);
        self.creation_timestamp(order_id).set(current_timestamp);

        self.create_order_event(
            order_id,
            caller,
            offered_token,
            offered_amount,
            offered_token_decimals,
            wanted_token,
            wanted_amount,
            wanted_token_decimals,
            current_timestamp,
        );
    }

    #[endpoint(cancelOrder)]
    fn cancel_order(
        &self,
        order_id: usize,
    ) {
        // check if SC is paused
        self.require_activation();

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(
            self.live_order_ids().contains(&order_id),
            "order_id not exist"
        );
        require!(
            caller == self.order_owner(order_id).get(),
            "you are not owner of the order"
        );

        self.live_order_ids().swap_remove(&order_id);

        //
        self.send().direct(
            &caller,
            &self.offered_token(order_id).get(),
            0,
            &self.offered_amount(order_id).get(),
            &[]
        );

        self.cancel_order_event(
            order_id,
            caller,
            current_timestamp,
        );
    }

    #[payable("*")]
    #[endpoint(acceptOrder)]
    fn accept_order(
        &self,
        #[payment_token] payment_token: TokenIdentifier,
        #[payment_amount] payment_amount: BigUint,
        order_id: usize,
        percentage: u64,    // 500 = 50%
    ) {
        // check if SC is paused
        self.require_activation();

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(
            self.live_order_ids().contains(&order_id),
            "order_id not exist"
        );
        require!(
            percentage <= TOTAL_PERCENTAGE,
            "percentage cannot be greater than 100%"
        );

        require!(
            payment_token == self.wanted_token(order_id).get(),
            "payment_token and wanted_token not match"
        );
        require!(
            payment_amount == self.wanted_amount(order_id).get() * percentage / TOTAL_PERCENTAGE,
            "payment_amount and percentage of wanted_amount not match"
        );

        let fee = self.fee().get();
        let offered_amount = self.offered_amount(order_id).get() * percentage / TOTAL_PERCENTAGE;

        let will_give_fee;
        let will_take_fee;

        if fee != 0 {
            will_give_fee = payment_amount.clone() * fee / TOTAL_PERCENTAGE;
            require!(
                will_give_fee != BigUint::zero(),
                "will_give_fee cannot be 0"
            );
            will_take_fee = offered_amount.clone() * fee / TOTAL_PERCENTAGE;
            require!(
                will_take_fee != BigUint::zero(),
                "will_take_fee cannot be 0"
            );
        } else {
            will_give_fee = BigUint::zero();
            will_take_fee = BigUint::zero();
        }

        let will_give_amount = payment_amount.clone() - &will_give_fee;
        let will_take_amount = offered_amount.clone() - &will_take_fee;

        // remove order if percentage is 100%
        if percentage == TOTAL_PERCENTAGE {
            self.live_order_ids().swap_remove(&order_id);
        } else {
            self.offered_amount(order_id).update(|v| *v -= &offered_amount);
            self.wanted_amount(order_id).update(|v| *v -= &payment_amount);

            // check if amount of left tokens is not zero
            require!(
                self.offered_amount(order_id).get() != BigUint::zero(),
                "left offered_amount cannot be zero"
            );
            require!(
                self.wanted_amount(order_id).get() != BigUint::zero(),
                "left wanted_amount cannot be zero"
            );
        }

        self.send().direct(
            &caller,
            &self.offered_token(order_id).get(),
            0,
            &will_take_amount,
            b"wanted token"
        );
        self.send().direct(
            &self.order_owner(order_id).get(),
            &payment_token,
            0,
            &will_give_amount,
            b"offered token"
        );

        if fee != 0 {
            let treasury_address = self.treasury_address().get();
            self.send().direct(
                &treasury_address,
                &self.offered_token(order_id).get(),
                0,
                &will_take_fee,
                b"wanted token fee"
            );
            self.send().direct(
                &treasury_address,
                &payment_token,
                0,
                &will_give_fee,
                b"offered token fee"
            );
        }

        self.accept_order_event(
            order_id,
            self.order_owner(order_id).get(),
            caller,
            percentage,
            current_timestamp,
        );
    }

    #[only_owner]
    #[endpoint(removeOldOrders)]
    fn remove_old_orders(&self) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let expiration_period = self.expiration_period().get();

        let mut remove_order_ids: ManagedVec<Self::Api, usize> = ManagedVec::new();
        for order_id in self.live_order_ids().iter() {
            if current_timestamp > self.creation_timestamp(order_id).get() + expiration_period {
                remove_order_ids.push(order_id);
            }
        }

        for order_id in remove_order_ids.iter() {
            self.live_order_ids().swap_remove(&order_id);

            //
            self.send().direct(
                &self.order_owner(order_id).get(),
                &self.offered_token(order_id).get(),
                0,
                &self.offered_amount(order_id).get(),
                &[]
            );

            self.cancel_order_event(
                order_id,
                caller.clone(),
                current_timestamp,
            );
        }
    }

    #[inline]
    fn require_activation(&self) {
        require!(
            !self.paused().get(),
            "smart contract is paused"
        );
    }
}