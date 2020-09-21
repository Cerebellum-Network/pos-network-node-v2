// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod trait_;
mod module;
mod call;
mod error;
mod event;
mod storage;
mod module_interface;
mod store_trait;
mod instances;
mod genesis_build;
mod genesis_config;
mod type_value;

use crate::pallet::Def;
use quote::ToTokens;

/// Expand definition, in particular:
/// * add some bounds and variants to type defined,
/// * create some new types,
/// * impl stuff on them.
pub fn expand(mut def: Def) -> proc_macro2::TokenStream {
	let trait_ = trait_::expand_trait_(&mut def);
	let module = module::expand_module(&mut def);
	let call = call::expand_call(&mut def);
	let error = error::expand_error(&mut def);
	let event = event::expand_event(&mut def);
	let storages = storage::expand_storages(&mut def);
	let instances = instances::expand_instances(&mut def);
	let store_trait = store_trait::expand_store_trait(&mut def);
	let module_interface = module_interface::expand_module_interface(&mut def);
	let genesis_build = genesis_build::expand_genesis_build(&mut def);
	let genesis_config = genesis_config::expand_genesis_config(&mut def);
	let type_values = type_value::expand_type_values(&mut def);

	let new_items = quote::quote!(
		#trait_
		#module
		#call
		#error
		#event
		#storages
		#instances
		#store_trait
		#module_interface
		#genesis_build
		#genesis_config
		#type_values
	);

	def.item.content.as_mut().expect("This is checked by parsing").1
		.push(syn::Item::Verbatim(new_items));

	def.item.into_token_stream()
}