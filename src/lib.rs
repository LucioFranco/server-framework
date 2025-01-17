//! Opinionated framework for running network servers.
//!
//! This crate builds on top of libraries like [`tower`], [`hyper`], [`axum`], and [`tonic`], and
//! provides an opinionated way to run services built with those libraries.
//!
//! This is how we run all our Rust network services at [Embark Studios].
//!
//! [Embark Studios]: https://www.embark-studios.com
//!
//! # Example
//!
//! ```rust
//! use server_framework::{Server, Config};
//! use axum::{Router, routing::get};
//!
//! // use the default config
//! let config = Config::default();
//!
//! // build our application with a few routes
//! let routes = Router::new()
//!     .route("/", get(|| async { "Hello, World!" }))
//!     .route("/foo", get(|| async { "Hi from `GET /foo`" }));
//!
//! # async {
//! // run our server
//! Server::new(config)
//!     .with(routes)
//!     .always_live_and_ready()
//!     .serve()
//!     .await
//!     .unwrap();
//! # };
//! ```
//!
//! # Middleware
//!
//! At its core `server-framework` is a collection of `tower` middleware that extends your app with
//! Embark's conventions and best practices.
//!
//! The middleware stack includes:
//!
//! - Timeouts
//! - Setting and propagating request id headers
//! - Metrics recording
//! - Tracing with OpenTelemetry support
//!
//! # Metrics and health checks
//!
//! [`Server::serve`] will also start a second HTTP server separate from your primary application
//! that serves metrics and health checks. The default URLs are:
//!
//! - `GET host:8081/metrics`
//! - `GET host:8081/health/live`
//! - `GET host:8081/health/ready`
//!
//! The port can be configred with [`Config::metrics_health_port`].
//!
//! # Features
//!
//! `server-framework` includes the following optional features:
//!
//! | Name | Description | Default |
//! |---|---|---|
//! | `tonic` | Enables support for running tonic services | Yes |

// BEGIN - Embark standard lints v5 for Rust 1.55+
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::disallowed_methods,
    clippy::disallowed_types,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::single_match_else,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.5 for Rust 1.55+
// crate-specific exceptions:
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs)]
#![deny(unreachable_pub, private_in_public)]
#![forbid(unsafe_code)]

pub use anyhow;
pub use axum;
pub use axum::async_trait;
pub use http;
#[cfg(feature = "tonic")]
pub use tonic;
pub use tower;

pub mod health;

mod config;
mod error_handling;
mod middleware;
mod request_id;
mod server;

use axum::body::BoxBody;

pub use self::{config::Config, server::Server};

/// Type alias for [`axum::Router`] with [`BoxBody`] as the request body type, which this crate
/// requires.
pub type Router<B = BoxBody> = axum::Router<B>;

/// Type alias for [`http::Request`] with [`BoxBody`] as the body type, which this crate requires.
pub type Request<B = BoxBody> = http::Request<B>;

#[doc(inline)]
pub use axum::response::Response;

pub mod metrics {
    //! Types and utilities for metrics.

    pub use metrics_exporter_prometheus::Matcher;
}
