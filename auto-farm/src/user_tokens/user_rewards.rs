use common_structs::PaymentsVec;
use mergeable::Mergeable;

use crate::common::{
    address_to_id_mapper::{AddressId, NULL_ID},
    rewards_wrapper::RewardsWrapper,
    unique_payments::UniquePayments,
};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait UserRewardsModule:
    crate::common::common_storage::CommonStorageModule
    + crate::fees::FeesModule
    + crate::external_sc_interactions::locked_token_merging::LockedTokenMergingModule
    + lkmex_transfer::energy_transfer::EnergyTransferModule
    + legacy_token_decode_module::LegacyTokenDecodeModule
    + energy_query::EnergyQueryModule
    + utils::UtilsModule
{
    #[endpoint(userClaimRewards)]
    fn user_claim_rewards_endpoint(&self) -> PaymentsVec<Self::Api> {
        let caller = self.blockchain().get_caller();
        let user_id = self.user_ids().get_id_non_zero(&caller);
        self.user_claim_rewards(caller, user_id)
    }

    fn user_claim_rewards(
        &self,
        user: ManagedAddress,
        user_id: AddressId,
    ) -> PaymentsVec<Self::Api> {
        let rewards_mapper = self.user_rewards(user_id);
        self.claim_common(user, rewards_mapper)
    }

    #[view(getUserRewards)]
    fn get_user_rewards_view(&self, user: ManagedAddress) -> RewardsWrapper<Self::Api> {
        let user_id = self.user_ids().get_id(&user);
        if user_id != NULL_ID {
            self.user_rewards(user_id).get()
        } else {
            RewardsWrapper::default()
        }
    }

    fn add_user_rewards(
        &self,
        user: ManagedAddress,
        user_id: AddressId,
        locked_tokens: UniquePayments<Self::Api>,
        other_tokens: UniquePayments<Self::Api>,
    ) {
        let opt_merged_locked_tokens =
            self.merge_locked_tokens(user.clone(), locked_tokens.into_payments());
        let mut rew_wrapper = RewardsWrapper {
            opt_locked_tokens: opt_merged_locked_tokens,
            other_tokens,
        };
        self.take_fees(user.clone(), &mut rew_wrapper);

        let rewards_mapper = self.user_rewards(user_id);
        if rewards_mapper.is_empty() {
            rewards_mapper.set(rew_wrapper);
            return;
        }

        rewards_mapper.update(|existing_wrapper| {
            if let Some(new_locked_tokens) = rew_wrapper.opt_locked_tokens {
                self.merge_wrapped_locked_tokens(user, existing_wrapper, new_locked_tokens);
            }

            existing_wrapper
                .other_tokens
                .merge_with(rew_wrapper.other_tokens);
        });
    }

    #[storage_mapper("userRewards")]
    fn user_rewards(&self, user_id: AddressId) -> SingleValueMapper<RewardsWrapper<Self::Api>>;
}
