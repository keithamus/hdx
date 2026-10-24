use csskit_source_finder::{VisitMode, VisitableNode, find_visitable_nodes};
use heck::{ToKebabCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
	collections::HashSet,
	env,
	fs::write,
	io::Error,
	path::{Path, PathBuf},
};
use syn::parse_str;

fn write_tokens(file: &str, source: TokenStream) -> Result<(), Error> {
	let contents = syn::parse_file(&source.to_string()).map_err(|e| Error::other(e.to_string()))?;
	let contents = prettyplease::unparse(&contents);
	write(Path::new(&env::var("OUT_DIR").unwrap()).join(file), contents)
}

fn main() {
	println!("cargo::rerun-if-changed=build.rs");

	// Find all visitable nodes (for Visit trait)
	let mut all_visitable = HashSet::<_>::new();
	find_visitable_nodes("src/**/*.rs", &mut all_visitable, |path: &PathBuf| {
		println!("cargo::rerun-if-changed={}", path.display());
	});
	let mut all_visitable = all_visitable.into_iter().collect::<Vec<_>>();
	// Include nodes from css_parse
	all_visitable.push(VisitableNode {
		input: parse_str("pub struct ComponentValues<'a>;").unwrap(),
		visit_mode: VisitMode::All,
	});
	all_visitable.push(VisitableNode {
		input: parse_str("pub enum ComponentValue<'a> {}").unwrap(),
		visit_mode: VisitMode::Self_,
	});
	all_visitable.push(VisitableNode {
		input: parse_str("pub enum AttributeOperator {}").unwrap(),
		visit_mode: VisitMode::Self_,
	});
	all_visitable.sort_unstable_by_key(|node| node.ident().to_string());

	let queryable = all_visitable.iter().filter(|node| node.visit_mode.is_queryable()).cloned().collect::<Vec<_>>();

	// NodeId enum - only queryable types
	{
		let count = queryable.len();
		let variants = queryable.iter().enumerate().map(|(idx, node)| {
			let ident = node.ident();
			let discriminant = idx as u16;
			quote! { #ident = #discriminant }
		});

		let type_names = queryable.iter().map(|node| {
			let ident = node.ident();
			let generics = node.generics();
			if generics.type_params().next().is_some() || generics.const_params().next().is_some() {
				let name = ident.to_string();
				return quote! { #name };
			}
			let ty = match generics.lifetimes().count() {
				0 => quote! { #ident },
				lifetimes => {
					let statics = std::iter::repeat_n(quote! { 'static }, lifetimes);
					quote! { #ident<#(#statics),*> }
				}
			};
			quote! { <crate::#ty as ::stable_type_layout::TypeLayout>::TYPE_LAYOUT.name }
		});

		let tag_names = queryable.iter().map(|node| node.ident().to_string().to_kebab_case());

		let mut tag_order = (0..count as u16).collect::<Vec<_>>();
		tag_order.sort_unstable_by_key(|index| queryable[*index as usize].ident().to_string().to_kebab_case());

		let all = queryable.iter().map(|node| {
			let ident = node.ident();
			quote! { NodeId::#ident }
		});

		#[rustfmt::skip]
		let source = quote! {
			/// Unique identifier for each AST node type that can be queried.
			///
			/// This enum is automatically generated from types that derive `Visitable`
			/// and have `#[visit]`, `#[visit(self)]`, or `#[visit(all)]` attributes.
			/// Each variant has a unique discriminant value assigned at build time.
			#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
			#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
			#[repr(u16)]
			pub enum NodeId {
				#(#variants),*
			}

			const TYPE_NAMES: &[&str] = &[#(#type_names),*];
			const TAG_NAMES: &[&str] = &[#(#tag_names),*];
			const TAG_ORDER: &[u16] = &[#(#tag_order),*];
			const ALL: &[NodeId] = &[#(#all),*];

			impl std::fmt::Display for NodeId {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					f.write_str(self.type_name())
				}
			}

			impl NodeId {
				/// The number of node kinds, i.e. the number of variants of this enum.
				pub const COUNT: usize = #count;

				/// Returns the kebab-case tag name for this node type.
				pub const fn tag_name(self) -> &'static str {
					TAG_NAMES[self as usize]
				}

				/// Attempts to parse a tag name string into a NodeId.
				pub fn from_tag_name(name: &str) -> Option<Self> {
					let order = TAG_ORDER.binary_search_by(|index| TAG_NAMES[*index as usize].cmp(name)).ok()?;
					Some(ALL[TAG_ORDER[order] as usize])
				}

				/// Returns the Rust type name for this node type (e.g., "StyleRule", "Declaration").
				pub const fn type_name(self) -> &'static str {
					TYPE_NAMES[self as usize]
				}

				/// Attempts to resolve a Rust type name into a NodeId.
				///
				/// Returns `None` if the type name doesn't match any known node type.
				pub fn from_type_name(name: &str) -> Option<Self> {
					Some(ALL[TYPE_NAMES.binary_search(&name).ok()?])
				}

				/// Resolves a discriminant back into its NodeId.
				///
				/// Returns `None` if the index is not valid.
				pub const fn from_index(index: u16) -> Option<Self> {
					if (index as usize) < Self::COUNT { Some(ALL[index as usize]) } else { None }
				}

				/// Returns an iterator over all possible NodeId values.
				pub fn all_variants() -> impl Iterator<Item = Self> {
					ALL.iter().copied()
				}
			}

			#[cfg(test)]
			mod node_id_tests {
				use super::{ALL, NodeId, TAG_NAMES, TAG_ORDER, TYPE_NAMES};

				#[test]
				fn every_name_round_trips() {
					for id in NodeId::all_variants() {
						assert_eq!(NodeId::from_type_name(id.type_name()), Some(id), "{}", id.type_name());
						assert_eq!(NodeId::from_tag_name(id.tag_name()), Some(id), "{}", id.tag_name());
						assert_eq!(NodeId::from_index(id as u16), Some(id), "{}", id.type_name());
					}
				}

				#[test]
				fn tables_are_indexed_by_discriminant() {
					assert_eq!(ALL.len(), NodeId::COUNT);
					assert_eq!(TYPE_NAMES.len(), NodeId::COUNT);
					assert_eq!(TAG_NAMES.len(), NodeId::COUNT);
					assert_eq!(TAG_ORDER.len(), NodeId::COUNT);
					for (index, id) in ALL.iter().enumerate() {
						assert_eq!(*id as usize, index);
					}
				}

				#[test]
				fn lookup_tables_are_sorted() {
					assert!(TYPE_NAMES.is_sorted(), "TYPE_NAMES must be sorted for from_type_name");
					assert!(
						TAG_ORDER.windows(2).all(|pair| TAG_NAMES[pair[0] as usize] < TAG_NAMES[pair[1] as usize]),
						"TAG_ORDER must be sorted by tag name, with no duplicate tags, for from_tag_name"
					);
				}

				#[test]
				fn unknown_names_are_none() {
					assert_eq!(NodeId::from_type_name("NotANodeType"), None);
					assert_eq!(NodeId::from_tag_name("not-a-node-type"), None);
					assert_eq!(NodeId::from_type_name(""), None);
					assert_eq!(NodeId::from_tag_name(""), None);
					assert_eq!(NodeId::from_index(NodeId::COUNT as u16), None);
				}
			}
		};
		write_tokens("css_node_kind.rs", source).unwrap()
	}

	// apply_visit_methods - all visitable types (visit_xxx AND exit_xxx methods)
	{
		let methods = all_visitable.iter().flat_map(|node| {
			let ident = node.ident();
			let method_name = node.ident().to_string().to_snake_case();
			let visit_method_name = format_ident!("visit_{}", method_name);
			let exit_method_name = format_ident!("exit_{}", method_name);
			let (impl_generics, ty_generics, _) = node.generics().split_for_impl();
			let type_params = node.generics().type_params().map(|param| &param.ident).collect::<Vec<_>>();
			[
				quote! { #visit_method_name #impl_generics (#ident #ty_generics) [#(#type_params),*] },
				quote! { #exit_method_name #impl_generics (#ident #ty_generics) [#(#type_params),*] },
			]
		});
		let source = quote! {
			macro_rules! apply_visit_methods {
				($macro: ident) => {
					$macro! {
						#(#methods,)*
					}
				}
			}
		};
		write_tokens("css_apply_visit_methods.rs", source).unwrap();
	}

	{
		let parse_arms = queryable
			.iter()
			.filter_map(|node| {
				let ident = node.ident();
				let generics = node.generics();
				if generics.type_params().next().is_some() || generics.const_params().next().is_some() {
					return None;
				}
				let ty = match generics.lifetimes().count() {
					0 => quote! { #ident },
					lifetimes => {
						let each = std::iter::repeat_n(quote! { 'a }, lifetimes);
						quote! { #ident<#(#each),*> }
					}
				};
				Some(quote! { NodeId::#ident => ErasedNodeParser::<#ty>(PhantomData).parse(arena, source) })
			})
			.collect::<Vec<_>>();
		#[rustfmt::skip]
		let source = quote! {
			/// Parses `source` as the node kind that `id` names, into `arena`.
			///
			/// Returns `None` if the kind has no standalone grammar.
			pub fn parse_root<'a>(id: NodeId, arena: &'a Arena, source: &'a str) -> Option<ParsedRoot<'a>> {
				match id {
					#(#parse_arms,)*
					_ => None,
				}
			}
		};
		write_tokens("css_root_dispatch.rs", source).unwrap();
	}

	{
		let mut vendor_atoms: Vec<proc_macro2::Ident> = Vec::new();
		let variants = all_visitable.iter().filter_map(|node| {
			let ident = node.ident();
			if matches!(
				ident.to_string().as_str(),
				"FontFaceRuleStyleValue"
					| "PropertyRuleStyleValue"
					| "CounterStyleRuleStyleValue"
					| "ColorProfileRuleStyleValue"
					| "FontFeatureValuesRuleStyleValue"
					| "FontPaletteValuesRuleStyleValue"
			) {
				return None;
			}
			node.ident().to_string().strip_suffix("StyleValue").and_then(|name| {
				let generics = node.generics();
				if name.is_empty() {
					return None;
				}
				let kebab = name.to_kebab_case();
				let variant_name =
					if matches!(kebab.split('-').next().unwrap_or_default(), "webkit" | "moz" | "ms" | "o") {
						vendor_atoms.push(format_ident!("_{name}"));
						format_ident!("_{name}")
					} else {
						format_ident!("{name}")
					};
				Some(quote! { #variant_name: #ident #generics = #variant_name })
			})
		});
		// Collect into a Vec so vendor_atoms is fully populated before we use it
		let variants: Vec<_> = variants.collect();
		let source = quote! {
			macro_rules! apply_properties {
				($macro: ident) => {
					$macro! {
						#(#variants,)*
					}
				}
			}

			/// Vendor-prefixed CSS property atoms with known [`StyleValue`] implementations.
			///
			/// AUTO-GENERATED by css_ast build.rs.
			pub const CSS_VENDOR_PROPERTY_ATOMS: &[crate::CssAtomSet] = &[
				#(crate::CssAtomSet::#vendor_atoms,)*
			];
		};
		write_tokens("css_apply_properties.rs", source).unwrap();
	}
}
