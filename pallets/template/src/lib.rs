#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}



	#[pallet::storage]
	pub(super) type NameOf<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<u8, T::MaxLength>>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		EstMercado {
			who: T::AccountId,
			produto: BoundedVec<u8, T::MaxLength>
		}

	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		TooLong,
	}

   #[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    #[pallet::weight(T::WeightInfo::do_something())]
    pub fn set_produto(origin: OriginFor<T>, nome: Vec<u8>, valor: u32) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Define status baseado no valor
        let status = if valor > 0 {
            b"disponivel".to_vec()
        } else {
            b"indisponivel".to_vec()
        };

        let mut fullproduto = nome.clone();
        fullproduto.extend_from_slice(b";");

        // Remover a linha de conversão do valor para bytes
        // let valor_bytes = valor.to_string().into_bytes();

        // fullproduto.extend_from_slice(&valor_bytes);
        fullproduto.extend_from_slice(b";");

        fullproduto.extend_from_slice(&status);

        // Tenta converter para BoundedVec e trata erro
        let produto: BoundedVec<_, _> =
            fullproduto.try_into().map_err(|_| Error::<T>::TooLong)?;
        Self::deposit_event(Event::EstMercado { who, produto });
        Ok(())
    }
}
}
