// This file is part of pallet-relaychain-info.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_runtime::traits::BlockNumberProvider;

use cumulus_primitives_core::PersistedValidationData;
// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::BlockNumberProvider;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Provider of relay chain block number
        type RelaychainBlockNumberProvider: BlockNumberProvider<BlockNumber = Self::BlockNumber>;
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Current block numbers
        /// [ Parachain block number, Relaychain Block number ]
        CurrentBlockNumbers(T::BlockNumber, T::BlockNumber),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

pub struct OnValidationDataHandler<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> cumulus_pallet_parachain_system::OnSystemEvent for OnValidationDataHandler<T> {
    fn on_validation_data(data: &PersistedValidationData) {
        crate::Pallet::<T>::deposit_event(crate::Event::CurrentBlockNumbers(
            frame_system::Pallet::<T>::current_block_number(),
            data.relay_parent_number.into(),
        ));
    }

    fn on_validation_code_applied() {
        unimplemented!()
    }
}
