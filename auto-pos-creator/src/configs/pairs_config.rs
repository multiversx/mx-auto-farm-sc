elrond_wasm::imports!();

pub struct PairConfig<M: ManagedTypeApi> {
    pub lp_token_id: TokenIdentifier<M>,
    pub first_token_id: TokenIdentifier<M>,
    pub second_token_id: TokenIdentifier<M>,
}

#[elrond_wasm::module]
pub trait PairsConfigModule: utils::UtilsModule {
    #[only_owner]
    #[endpoint(addPairsToWhitelist)]
    fn add_pairs_to_whitelist(&self, pair_addresses: MultiValueEncoded<ManagedAddress>) {
        for pair_address in pair_addresses {
            self.require_sc_address(&pair_address);

            let config = self.get_pair_config(&pair_address);
            let token_pair_mapper =
                self.pair_address_for_tokens(&config.first_token_id, &config.second_token_id);
            require!(token_pair_mapper.is_empty(), "Token pair already known");

            token_pair_mapper.set(&pair_address);
            self.pair_for_lp_token(&config.lp_token_id)
                .set(&pair_address);
        }
    }

    #[only_owner]
    #[endpoint(removePairsFromWhitelist)]
    fn remove_pairs_from_whitelist(&self, pair_addresses: MultiValueEncoded<ManagedAddress>) {
        for pair_address in pair_addresses {
            self.require_sc_address(&pair_address);

            let config = self.get_pair_config(&pair_address);
            self.pair_address_for_tokens(&config.first_token_id, &config.second_token_id)
                .clear();
            self.pair_for_lp_token(&config.lp_token_id).clear();
        }
    }

    fn get_pair_config(&self, pair_address: &ManagedAddress) -> PairConfig<Self::Api> {
        let lp_token_id = self.lp_token_identifier().get_from_address(pair_address);
        let first_token_id = self.first_token_id().get_from_address(pair_address);
        let second_token_id = self.second_token_id().get_from_address(pair_address);

        self.require_valid_token_id(&lp_token_id);
        self.require_valid_token_id(&first_token_id);
        self.require_valid_token_id(&second_token_id);

        PairConfig {
            lp_token_id,
            first_token_id,
            second_token_id,
        }
    }

    fn get_pair_address_for_tokens(
        &self,
        first_token_id: &TokenIdentifier,
        second_token_id: &TokenIdentifier,
    ) -> ManagedAddress {
        let correct_order_mapper = self.pair_address_for_tokens(first_token_id, second_token_id);
        if !correct_order_mapper.is_empty() {
            return correct_order_mapper.get();
        }

        let reverse_order_mapper = self.pair_address_for_tokens(second_token_id, first_token_id);
        require!(!reverse_order_mapper.is_empty(), "No pair for given tokens");

        reverse_order_mapper.get()
    }

    #[storage_mapper("pairAddrForTokens")]
    fn pair_address_for_tokens(
        &self,
        first_token_id: &TokenIdentifier,
        second_token_id: &TokenIdentifier,
    ) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("pairForLpToken")]
    fn pair_for_lp_token(&self, lp_token_id: &TokenIdentifier)
        -> SingleValueMapper<ManagedAddress>;

    // Pair storage

    #[storage_mapper("lpTokenIdentifier")]
    fn lp_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("first_token_id")]
    fn first_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("second_token_id")]
    fn second_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}