use crate::{plugin_for_implementors_of_trait, prelude::*};

pub struct GenericPlugins;

plugin_for_implementors_of_trait!(NumericPlugins, Numeric);

impl Plugin for GenericPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            NumericPlugins::<f32>::default(),
            NumericPlugins::<Vec2>::default(),
            NumericPlugins::<Vec3>::default(),
            NumericPlugins::<Quat>::default(),
        ));
    }
}

impl<T: Numeric> Plugin for NumericPlugins<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TimerGoingEventPlugin::<T>::default(),
            GoingEventEmittingPlugin::<T>::default(),
            GoingEventValueCalculatorsPlugin::<T>::default(),
            AffectingTimerCalculatorsPlugin::<T>::default(),
            TimerFiringGenericPlugin::<T>::default(),
        ));
    }
}
