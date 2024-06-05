#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod error_example {

    #[ink(storage)]
    pub struct ErrorExample {
        value: u128,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum Error {
        DataShouldNotBeZero,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl ErrorExample {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 0 }
        }

        #[ink(message)]
        pub fn get_value(&self) -> u128 {
            self.value
        }

        #[ink(message)]
        pub fn set_value(&mut self, new_value: u128) -> Result<()> {
            if new_value == 0 {
                return Err(Error::DataShouldNotBeZero);
            }
            self.value = new_value;
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut error_example = ErrorExample::new();
            assert_eq!(error_example.get_value(), 0);
            let _ = error_example.set_value(2);
            assert_eq!(error_example.get_value(), 2);
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
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;


        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = ErrorExampleRef::new();
            let contract = client
                .instantiate("error_example", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<ErrorExample>();

            let get = call_builder.get_value();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), 0));

            // When
            let flip = call_builder.set_value(2);
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get_value();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), 2));

            Ok(())
        }
    }
}
