use std::{
  any::{Any, TypeId},
  collections::HashMap,
  hash::Hash,
  sync::Arc,
};

/// A per-request cache keyed by resource function type and input value.
///
/// Each resource function owns a slot in the map, discriminated by
/// `TypeId::of::<F>()`. Within each slot, results are keyed by input value.
/// Outputs are stored behind `Arc` so neither the output nor the error type
/// need to implement `Clone`.
pub(crate) struct RequestCache(HashMap<TypeId, Box<dyn Any + Send>>);

impl RequestCache {
  pub(crate) fn new() -> Self { Self(HashMap::new()) }

  /// Returns the cached output for `(F, input)`, if present.
  pub(crate) fn get<F: 'static, I: Hash + Eq + 'static, O: 'static>(
    &self,
    input: &I,
  ) -> Option<Arc<O>> {
    self
      .0
      .get(&TypeId::of::<F>())?
      .downcast_ref::<HashMap<I, Arc<O>>>()?
      .get(input)
      .cloned()
  }

  /// Stores an output for `(F, input)`.
  pub(crate) fn insert<
    F: 'static,
    I: Hash + Eq + Send + 'static,
    O: Send + Sync + 'static,
  >(
    &mut self,
    input: I,
    output: Arc<O>,
  ) {
    self
      .0
      .entry(TypeId::of::<F>())
      .or_insert_with(|| Box::new(HashMap::<I, Arc<O>>::new()))
      .downcast_mut::<HashMap<I, Arc<O>>>()
      .unwrap()
      .insert(input, output);
  }
}
