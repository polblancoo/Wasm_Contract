#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod staking {
    
    use dapps_staking_extension::*;
    use ink::storage::Mapping;
    /// Event emitted when a value is staked
    #[ink(event)]
    pub struct Staked {
        #[ink(topic)]
        account: AccountId,
        era: u32,
        amount: Balance,
    }

    /// Event emitted when a value is unstaked
    #[ink(event)]
    pub struct Unstaked {
        #[ink(topic)]
        account: AccountId,
        era: u32,
        amount: Balance,
    }
    /// Defines the storage of your contract.
    
    #[ink(storage)]
    pub struct StakingContract {
        /// Stores a single `bool` value on the storage.
        stakers: Mapping<AccountId, Balance>,
    }
    /// Errors occurred in the contract
    #[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        TransferError,
        AddOverFlow,
        SubOverFlow,
        DSError(DSError)
    }
    /// convertor from DSError to ContractError
    impl From<DSError> for ContractError {
        fn from(error: DSError) -> Self {
            ContractError::DSError(error)
        }
    }

    impl StakingContract {
        /// Constructor that initializes .
        #[ink(constructor)]
        pub fn default() -> Self {
            Self { stakers: Mapping::default() }
        }

        

        #[ink(message)]
        pub fn read_current_era(&self) -> u32 {
            DappsStaking::read_current_era()
        }

        #[ink(message)]
        pub fn get_staked_amount(&self, account: AccountId) -> Option<Balance> {
            self.stakers.get(&account)
        }

         /// read the amount staked on this contract by this contract
         #[ink(message)]
         pub fn read_staked_amount_on_contract(&self) -> Balance {
             let contract = self.env().account_id();
             // read the amount staked on this contract by this contract
             DappsStaking::read_staked_amount_on_contract(contract, contract)
         }

          /// read the total amount staked on this contract
        #[ink(message)]
        pub fn read_contract_stake(&self) -> Balance {
            let contract = self.env().account_id();
            DappsStaking::read_contract_stake(contract)
        }

        #[ink(message, payable)]
        pub fn bond_and_stake(&mut self) -> Result<(), ContractError> {

            let caller = self.env().caller();
            let value = self.env().transferred_value();

            // compute the new stake
            let new_stake = match self.stakers.get(&caller){
                Some(existing) => {existing.checked_add(value).ok_or(ContractError::AddOverFlow)?}
                _ => {value}
            };

            // save the new amount staked by the caller
            self.stakers.insert(&caller, &new_stake);

            // Stake on this contract.
            let contract = self.env().account_id();
            // Here the staker will be the contract for the pallet dAppStaking
            DappsStaking::bond_and_stake(contract, value)?;

            // get the current era
            let era = DappsStaking::read_current_era();

            // emmit the event
            self.env().emit_event(Staked { account: caller, era, amount: value });

            Ok(())
        }
        #[ink(message)]
        pub fn unbond_and_unstake(&mut self, value: Balance) -> Result<(), ContractError> {
            let caller = self.env().caller();

            // compute the new stake
            let new_stake = match self.stakers.get(&caller){
                Some(existing) => {existing.checked_sub(value).ok_or(ContractError::SubOverFlow)?}
                _ => {value}
            };

            // save the new amount staked by the caller
            if new_stake == 0 {
                self.stakers.remove(&caller);
            } else {
                self.stakers.insert(&caller, &new_stake);
            }

            // Unbond and unstake on the contract
            let contract = self.env().account_id();
            DappsStaking::unbond_and_unstake(contract, value)?;

            // get back the fund to the user but normally we should respect the unbounding period
            // it means this method could fail if the fund are still locked
            self.env().transfer(caller, value).map_err(|_| ContractError::TransferError)?;

            // get the current era
            let era = DappsStaking::read_current_era();

            // emmit the event
            self.env().emit_event(Unstaked { account: caller, era, amount: value });

            Ok(())
        }

        #[ink(message)]
        pub fn withdraw_unbonded(&mut self) -> Result<(), ContractError> {
            DappsStaking::withdraw_unbonded()?;
            Ok(())
        }


    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
           // let staking = Staking::default();
           // assert_eq!(staking.get(), false);
        }

       
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = StakingRef::default();

            // When
            let contract_account_id = client
                .instantiate("staking", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<StakingRef>(contract_account_id.clone())
                .call(|staking| staking.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = StakingRef::new(false);
            let contract_account_id = client
                .instantiate("staking", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<StakingRef>(contract_account_id.clone())
                .call(|staking| staking.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<StakingRef>(contract_account_id.clone())
                .call(|staking| staking.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<StakingRef>(contract_account_id.clone())
                .call(|staking| staking.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
