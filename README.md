# [![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/sifis-home/sifis-ext/workflows/sifis-td/badge.svg)](https://github.com/sifis-home/sifis-ext/actions)
[![Crates.io](https://img.shields.io/crates/v/sifis-td.svg)](https://crates.io/crates/sifis-td)
[![dependency status](https://deps.rs/repo/github/sifis-home/sifis-ext/status.svg)](https://deps.rs/repo/github/sifis-home/sifis-ext)
[![Documentation](https://docs.rs/sifis-td/badge.svg)](https://docs.rs/sifis-td/)

Extension crate for [wot-td](https://crates.io/crates/wot-td) with the [SIFIS-Home](https://sifis-home.eu) [Hazards Ontology](https://purl.org/sifis/hazards).

## Rationale

This crates provides to ability to extend the _interaction affordance_ of a
_Thing description_ with a set of hazards. Each hazard identifies a specific
risk that can occur in a certain state of the _Thing_ or with certain behaviors.

The set of hazards are fixed and they all have a set of intrinsic
characteristics that do not depend on the _Thing_. On the other hand, some other
characteristics depend on the _Thing_ and the specific property or action the
hazard is referring to.

With this approach it is possible to unambiguously classify the set of hazards
for any _interaction affordance_ giving a different level of risk for different
ranges of the affordance.

Take for instance an halogen lamp, which has a _brightness_ property. In this
case, there is an associated _fire hazard_ that has a multiple sets of risk
levels depending on the level of _brightness_. Obviously, this depends on the
type of the lamp and its characteristics, even if the _fire hazard_ is the same
for all lamps.

Another example is a camera, which has an intrinsic risk for the privacy once
turned on. In this case, the level of risk has a fixed value but it is
meaningful only when the camera is turned on.

## Acknowledgements

This software has been developed in the scope of the H2020 project SIFIS-Home with GA n. 952652.
