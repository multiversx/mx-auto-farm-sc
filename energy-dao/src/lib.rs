#![no_std]

multiversx_sc::imports!();

use locked_token_wrapper::wrapped_token;

pub mod common;
pub mod external_sc_interactions;

#[multiversx_sc::contract]
pub trait EnergyDAO:
    external_sc_interactions::energy_dao_config::EnergyDAOConfigModule
    + external_sc_interactions::farm_actions::FarmActionsModule
    + external_sc_interactions::farm_interactions::FarmInteractionsModule
    + external_sc_interactions::metastaking_actions::MetastakingActionsModule
    + external_sc_interactions::metastaking_interactions::MetastakingInteractionsModule
    + external_sc_interactions::locked_token_actions::LockedTokenModule
    + external_sc_interactions::locked_token_interactions::LockedTokenInteractionsModule
    + external_sc_interactions::fees_collector_interactions::FeesCollectorInteractionsModule
    + lkmex_transfer::energy_transfer::EnergyTransferModule
    + legacy_token_decode_module::LegacyTokenDecodeModule
    + energy_query::EnergyQueryModule
    + token_send::TokenSendModule
    + utils::UtilsModule
    + wrapped_token::WrappedTokenModule
    + simple_lock::token_attributes::TokenAttributesModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(
        &self,
        energy_factory_address: ManagedAddress,
        fees_collector_sc_address: ManagedAddress,
        locked_token_wrapper_sc_address: ManagedAddress,
        exit_penalty_percent: u64,
        unbond_period: u64,
    ) {
        self.require_sc_address(&energy_factory_address);
        self.require_sc_address(&fees_collector_sc_address);
        self.require_sc_address(&locked_token_wrapper_sc_address);

        self.energy_factory_address()
            .set_if_empty(energy_factory_address);
        self.fees_collector_sc_address()
            .set_if_empty(fees_collector_sc_address);
        self.locked_token_wrapper_sc_address()
            .set_if_empty(locked_token_wrapper_sc_address);
        self.exit_penalty_percent()
            .set_if_empty(exit_penalty_percent);
        self.unbond_period().set_if_empty(unbond_period);
    }
}