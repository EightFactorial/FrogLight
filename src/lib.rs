//! <h1 align="center">FrogLight</h1>
//! <p align="center">A collection of Minecraft libraries written in Rust with support for <a href="https://bevyengine.org/">Bevy</a></p>
//!
//! <p align="center">
//!   <a href="https://github.com/EightFactorial/FrogLight"><img alt="Documentation" src="https://img.shields.io/badge/docs-main-green.svg"></a>
//!   <img alt="License" src="https://img.shields.io/badge/license-MIT/Apache--2.0---?color=blue">
//!   <a href="https://github.com/EightFactorial/FrogLight/actions"><img alt="Tests" src="https://github.com/EightFactorial/FrogLight/actions/workflows/testing.yml/badge.svg"></a>
//! </p>
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

// Re-export everything from `froglight-internal`
pub use froglight_internal::*;
