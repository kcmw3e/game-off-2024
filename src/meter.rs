//! Game-off 2024, Casey Walker and Nico Murolo
//! ----------------------------------------------------------------------------
//! This is the generic solution for "metered" components. Mostly, these are for
//! tracking health and mana, and since they are closely related in terms of how
//! they get tracked, used, etc., the implementation for them is the same and is
//! abstracted into the [`Meter`] type.
//!
//! [`Meter`]s can be used as Bevy [`Component`]s by giving them a
//! [`MeterMarker`], which is just an empty `struct` used to uniquely
//! differentiate between types of meteres so that they can be used separately
//! with Bevy.
//!
//! # Examples
//!
//! Here's an example of how to create a health-tracking meter:
//!
//! ```rust
//! use bevy::prelude::*;
//!
//! use crate::meter::{Meter, MeterMarker};
//!
//! struct HealthMarker {}
//! impl MeterMarker for HealthMarker {
//!     type Field = i64;
//! }
//!
//! type HealthMeter = Meter<HealthMarker>;
//!
//! fn setup_player(mut commands: Commands) {
//!     commands.spawn((
//!         HealthMeter::new_from_max(100),
//!     ));
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_systems(Startup, setup_player)
//!         .run();
//! }
//! ```
use std::marker::PhantomData;

use bevy::prelude::*;

/// A [`Meter`] represents some form of expendable resource for an entity. The
/// typical example of a meter is health and mana. In order to make [`Meter`]s
/// unique from the Bevy [`Component`] standpoint, a marker must be supplied in
/// the form of a struct implementing the [`MeterMarker`] trait which also
/// defines what type the [`Meter`] should track (e.g. [`i64`]).
#[derive(Component)]
pub struct Meter<T: MeterMarker> {
    /// The maximum amount storable in the bar.
    pub max: T::Field,
    /// The current amount stored in the bar.
    pub current: T::Field,
    /// The marker that uniquely defines what kind of meter.
    _marker: PhantomData<T>,
}

impl<T: MeterMarker> Meter<T> {
    /// Create a new [`Self`] with its current amount initialized to its
    /// maximum.
    pub fn new_from_max(max: T::Field) -> Self {
        return Self {
            max: max,
            current: max,
            _marker: PhantomData,
        };
    }
}

/// A [`MeterMarker`] can be used to create new, unique [`Meter`]s, each of
/// which can be used in Bevy as its own component.
pub trait MeterMarker: {
    /// The type of the meter's fields, typically [`i64`] or [`i32`].
    type Field: Copy;
}
