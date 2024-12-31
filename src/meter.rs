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
//!
//! Here's an example of how to create a time-based fire damage:
//!
//! ```rust
//! use bevy::prelude::*;
//!
//! mod meter;
//! use crate::meter::{Meter, MeterEffect, MeterEffectMarker, MeterMarker};
//!
//! struct HealthMarker {}
//! impl MeterMarker for HealthMarker {
//!     type Field = i64;
//! }
//!
//! struct FireDamageMarker {}
//! impl MeterEffectMarker for FireDamageMarker {
//!     type Marker = HealthMarker;
//! }
//!
//! type HealthMeter = Meter<HealthMarker>;
//! type FireDamage = MeterEffect<FireDamageMarker>;
//!
//! fn setup_player(mut commands: Commands) {
//!     commands.spawn((HealthMeter::new_from_max(100), FireDamage::new(-5)));
//! }
//!
//! fn main() {
//!     App::new()
//!         .insert_resource(Time::<Fixed>::from_seconds(1.0))
//!         .add_systems(Startup, setup_player)
//!         .add_systems(FixedUpdate, FireDamage::apply_effect)
//!         .run();
//! }
//! ```
use std::marker::PhantomData;

use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use num_traits::NumAssignOps;

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

/// A [`MeterEffect`] represents some type of value change on a meter's
/// `current` field. This can be used to represent things like one-time hits or
/// some mana usage from casing a spell, or perhaps debuffs.
#[derive(Component)]
pub struct MeterEffect<T: MeterEffectMarker> {
    /// The amount by which the meter should be changed (may be posititve or
    /// negative).
    amount: <T::Marker as MeterMarker>::Field,
    _marker: PhantomData<T>,
}

/// This custom query is used for querying an entity which has a meter and some
/// active effect.
///
/// It seems to be necessary to use this custom query instead of the typical way
/// of writing Bevy systems due to an issue with the way Rust interprets the
/// generic types during the definition (where an error specifying that the
/// `QueryData` trait is not satisfied.
#[derive(QueryData)]
#[query_data[mutable]]
pub struct MeterEffectQuery<T: MeterEffectMarker + 'static> {
    /// The meter that should be altered by the effect.
    meter: &'static mut Meter<T::Marker>,
    /// The effect that alters the meter.
    effect: &'static mut MeterEffect<T>,
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

impl<T: MeterEffectMarker> MeterEffect<T> {
    /// Create a new [`Self`] given an amount by which to change the associated
    /// meter.
    pub fn new(amount: <T::Marker as MeterMarker>::Field) -> Self {
        return Self {
            amount: amount,
            _marker: PhantomData,
        };
    }

    /// The Bevy system used to apply a meter effect. Note that the effect will
    /// be neither removed nor timed.
    pub fn apply_effect(mut query: Query<MeterEffectQuery<T>>) {
        for MeterEffectQueryItem { mut meter, effect } in query.iter_mut() {
            meter.current += effect.amount;
        }
    }
}

/// A [`MeterMarker`] can be used to create new, unique [`Meter`]s, each of
/// which can be used in Bevy as its own component.
pub trait MeterMarker: Send + Sync {
    /// The type of the meter's fields, typically [`i64`] or [`i32`].
    type Field: Copy + Send + Sync + NumAssignOps;
}

/// A [`MeterEffectMarker`] can be used to create new, unique [`MeterEffect`]s,
/// each of which can be used in Bevy as its own component.
pub trait MeterEffectMarker: Send + Sync {
    /// The type of the meter effect's meter marker (which will be used to
    /// define/query for the corresponding meter component).
    type Marker: MeterMarker;
}
