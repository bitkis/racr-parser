//! Rust parser for the racr format.
//!
//! # Examples
//! ## Parse Access
//! ```
//! assert_eq!(
//!     racr::Access::ReadOnly,
//!     racr_parser::AccessParser::new().parse("ReadOnly").unwrap()
//! );
//! ```
//! ```
//! assert_eq!(
//!     racr::Access::WriteOnly,
//!     racr_parser::AccessParser::new().parse("WriteOnly").unwrap()
//! );
//! ```
//! ```
//! assert_eq!(
//!     racr::Access::ReadWrite,
//!     racr_parser::AccessParser::new().parse("ReadWrite").unwrap()
//! );
//! ```
//! 
//! ## Parse paths
//! ```
//! assert_eq!(
//!     racr::Path{segments: vec!["foo".into(), "bar".into(), "baz".into()]},
//!     racr_parser::PathParser::new().parse("foo::bar::baz").unwrap()
//! );
//! ```
//! 
//! ## Parse modules
//! ```
//! assert_eq!(
//!     racr::Module{ident: "foo".into(), content: None},
//!     racr_parser::ModuleParser::new().parse("mod foo;").unwrap()
//! );
//! ```
//! ```
//! assert_eq!(
//!     racr::Module{ident: "foo".into(), content: Some(vec![
//!         racr::Module{ident: "bar".into(), content: Some(vec![
//!             racr::Module{ident: "baz".into(), content: None}.into()
//!         ])}.into()
//!     ])},
//!     racr_parser::ModuleParser::new().parse("mod foo {mod bar {mod baz;}}").unwrap()
//! );
//! ```
//! ## Parse use
//! ```
//! assert_eq!(
//!     racr::Use{tree: racr::UseTree::Ident("Foo".into())},
//!     racr_parser::UseParser::new().parse("use Foo;").unwrap()
//! );
//! ```
//! ```
//! assert_eq!(
//!     racr::Use{tree: racr::UseTree::Path{path_segment: "foo".into(), sub_tree:
//!         Box::new( racr::UseTree::Path{path_segment: "bar".into(), sub_tree: 
//!             Box::new( racr::UseTree::Ident("Baz".into()) )
//!         } )
//!     }},
//!     racr_parser::UseParser::new().parse("use foo::bar::Baz;").unwrap()
//! );
//! ```
//! ## Parse register definitions
//! ```
//! assert_eq!(
//!     racr::RegisterDefinition{
//!         access: racr::Access::WriteOnly,
//!         ident: "Foo".into(),
//!         description: Some(String::from("Some description")),
//!         size: 32,
//!         reset_value: Some(0x00),
//!         fields: vec![
//!             racr::FieldInstance{ident: "bar".into(), description: None, bit_start: 0, bit_end: 7, access: None},
//!             racr::FieldInstance{ident: "baz".into(), description: None, bit_start: 8, bit_end: 8, access: Some(racr::Access::ReadOnly)},
//!             racr::FieldInstance{ident: "bax".into(), description: Some(String::from("Some description")), bit_start: 9, bit_end: 31, access: None},
//!         ],
//!     },
//!     racr_parser::RegisterDefinitionParser::new().parse(" 
//! #[doc = \"Some description\"]
//! WriteOnly register[32] Foo = 0 {
//!    bar[0..7],
//!    ReadOnly baz[8],
//!    #[doc = \"Some description\"]
//!    bax[9..31],
//! }"
//!     ).unwrap()
//! );
//! ```
//! ## Parse peripheral definitions
//! ```
//! assert_eq!(
//!     racr::PeripheralDefinition{
//!         ident: "Foo".into(),
//!         description: Some(String::from("Some description")),
//!         registers: vec![
//!             racr::RegisterSlot::Single{instance: racr::RegisterInstance{ident: "bar".into(), ty: racr::RegisterType::Single{path: racr_parser::PathParser::new().parse("bar::Bar").unwrap()}}, offset: 0x0},
//!             racr::RegisterSlot::Single{instance: racr::RegisterInstance{ident: "bax".into(), ty: racr::RegisterType::Array{path: racr_parser::PathParser::new().parse("bax::Bax").unwrap(), size: 2}}, offset: 0x4},
//!             racr::RegisterSlot::Overloaded{
//!                 alternatives: vec![
//!                     racr::RegisterInstance{ident: "baz1".into(), ty: racr::RegisterType::Single{path: racr_parser::PathParser::new().parse("baz::Baz1").unwrap()}},
//!                     racr::RegisterInstance{ident: "baz2".into(), ty: racr::RegisterType::Single{path: racr_parser::PathParser::new().parse("baz::Baz2").unwrap()}},
//!                     racr::RegisterInstance{ident: "baz3".into(), ty: racr::RegisterType::Single{path: racr_parser::PathParser::new().parse("baz::Baz3").unwrap()}},
//!                 ],
//!                 offset: 0x10,
//!             },
//!         ],
//!     },
//!     racr_parser::PeripheralDefinitionParser::new().parse(" 
//! #[doc = \"Some description\"]
//! peripheral Foo {
//!    bar: bar::Bar @ 0x00,
//!    bax: [bax::Bax; 2] @ 0x04,
//!    (baz1: baz::Baz1 | baz2: baz::Baz2 | baz3: baz::Baz3) @ 0x10,
//! }"
//!     ).unwrap()
//! );
//! ```
//! ## Parse device definitions
//! ```
//! assert_eq!(
//!     racr::DeviceDefinition{
//!         ident: "Foo".into(),
//!         description: Some(String::from("Some description")),
//!         peripherals: vec![
//!             racr::PeripheralInstance{ident: "bar".into(), path: racr_parser::PathParser::new().parse("bar::Bar").unwrap(), address: 0x0},
//!             racr::PeripheralInstance{ident: "baz".into(), path: racr_parser::PathParser::new().parse("baz::Baz").unwrap(), address: 0x4},
//!             racr::PeripheralInstance{ident: "bax".into(), path: racr_parser::PathParser::new().parse("bax::Bax").unwrap(), address: 0xc},
//!         ],
//!     },
//!     racr_parser::DeviceDefinitionParser::new().parse(" 
//! #[doc = \"Some description\"]
//! device Foo {
//!    bar: bar::Bar @ 0x00,
//!    baz: baz::Baz @ 0x04,
//!    bax: bax::Bax @ 0x0c,
//! }"
//!     ).unwrap()
//! );
//! ```
//! ## Parse content
//! ```
//! racr_parser::ContentParser::new().parse("
//! use Foo;
//! use crate::bar::Baz;
//! 
//! mod module {
//!     peripheral Peripheral {
//!         foo: Foo @ 0x00,
//!         nar: Baz @ 0x10,
//!     }
//! }
//! ").unwrap();
//! ```

use lalrpop_util;
use lalrpop_util::lalrpop_mod;

use racr;

lalrpop_mod!(parser);

pub use crate::parser::ModuleParser;
pub use crate::parser::PathParser;
pub use crate::parser::AccessParser;
pub use crate::parser::ItemParser;
pub use crate::parser::UseParser;
pub use crate::parser::RegisterDefinitionParser;
pub use crate::parser::PeripheralDefinitionParser;
pub use crate::parser::DeviceDefinitionParser;
pub use crate::parser::ContentParser;
