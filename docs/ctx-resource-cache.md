# Design: Generalized Per-Request Resource Cache on `Ctx`

## The Problem

`CtxShared` has a hand-rolled `org_fetch_cache: Mutex<HashMap<RecordId<Org>, Option<Org>>>` and a
`fetch_org()` method on `Ctx`. Every new resource you want to cache needs a new field and a new
method — and the cache is only for simple model fetches. More complex resources like
`fetch_entry_count_for_cache` (which does an auth check, then a separate count query) are called
directly with no caching at all.

The goal is a single generic mechanism that works for **any async resource function**, regardless
of input type or output type.

---

## Key Insight: Fn Items as Cache Discriminants

In Rust, every `fn` item has a unique anonymous zero-sized type. That means
`TypeId::of::<F>()` is a stable, distinct key for each specific function — even if two functions
share the same signature. This is the discriminant we'll use to namespace cache entries.

Combined with a typed input key `I: Hash + Eq`, we get a cache address of
`(TypeId::of::<F>(), input)` with no boilerplate and no trait impls per resource.

---

## Design

### 1. `RequestCache`

A TypeMap where each slot is owned by one resource function.

```rust
// crates/app/src/request_cache.rs

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::Hash,
};

pub struct RequestCache(HashMap<TypeId, Box<dyn Any + Send>>);

impl RequestCache {
    pub fn new() -> Self { Self(HashMap::new()) }

    /// Looks up a previously cached result for `(F, input)`.
    pub fn get<F: 'static, I: Hash + Eq + 'static, O: 'static>(
        &self,
        input: &I,
    ) -> Option<&O> {
        self.0
            .get(&TypeId::of::<F>())?
            .downcast_ref::<HashMap<I, O>>()?
            .get(input)
    }

    /// Stores a result for `(F, input)`.
    pub fn insert<F: 'static, I: Hash + Eq + Send + 'static, O: Send + 'static>(
        &mut self,
        input: I,
        output: O,
    ) {
        self.0
            .entry(TypeId::of::<F>())
            .or_insert_with(|| Box::new(HashMap::<I, O>::new()))
            .downcast_mut::<HashMap<I, O>>()
            .unwrap()
            .insert(input, output);
    }
}
```

---

### 2. `CtxShared` — Replace the org field

```rust
struct CtxShared {
    app_state:      AppState,
    suspense_ctx:   SuspenseContext,
    auth_session:   AuthSession,
    request_cache:  Mutex<RequestCache>,   // replaces org_fetch_cache
}
```

---

### 3. `Ctx::fetch_cached` — The single generic entry point

Resource functions follow the shape `async fn(Ctx<A>, I) -> O`. The return type `O` is fully
opaque — it can be a `Result`, an `Option`, a plain value, anything. The output is cached
unconditionally on every call; callers own any error-handling semantics inside their resource
function.

```rust
impl<Auth: Clone + Send + Sync + 'static> Ctx<Auth> {
    pub async fn fetch_cached<F, I, O, Fut>(
        &self,
        resource_fn: F,
        input: I,
    ) -> O
    where
        F: FnOnce(Ctx<Auth>, I) -> Fut + 'static,
        I: Hash + Eq + Clone + Send + 'static,
        O: Clone + Send + 'static,
        Fut: Future<Output = O> + Send,
    {
        {
            let cache = self.0.shared.request_cache.lock().await;
            if let Some(cached) = cache.get::<F, I, O>(&input) {
                return cached.clone();
            }
        }

        let output = resource_fn(self.clone(), input.clone()).await;

        {
            let mut cache = self.0.shared.request_cache.lock().await;
            cache.insert::<F, I, O>(input, output.clone());
        }

        output
    }
}
```

---

### 4. Resource Functions

Resource functions are plain `async fn`s. The only convention is that they take `(Ctx<A>, I)`.
For zero-input resources, `I = ()`. The return type is whatever makes sense for that resource.

```rust
// crates/app/src/resources.rs

// Zero-input: add a `()` parameter to fit the convention
pub async fn fetch_caches_for_requested_org(
    ctx: Ctx<RequireRequestedOrg>,
    _: (),
) -> Result<Vec<PvCache>, DatabaseError> { ... }

// Already fits — ctx + typed input, Result return
pub async fn fetch_entry_count_for_cache(
    ctx: Ctx<RequireAuth>,
    cache_id: RecordId<Cache>,
) -> Result<Option<AuthResult<u64>>, DatabaseError> { ... }
```

For simple model fetches, small free functions live in `resources.rs`:

```rust
pub async fn fetch_org(
    ctx: Ctx<RequireAuth>,
    id: RecordId<Org>,
) -> Result<Option<Org>, DatabaseError> {
    ctx.state().domain.meta().fetch_org_by_id(id).await
}
```

This replaces the `fetch_org` method on `Ctx` and the bespoke `org_fetch_cache` field.

---

### 5. Call Sites

```rust
// Simple model fetch — was ctx.fetch_org(org_id)
ctx.fetch_cached(fetch_org, org_id).await

// Complex resource — was fetch_entry_count_for_cache(ctx, cache_id) with no caching
ctx.fetch_cached(fetch_entry_count_for_cache, cache_id).await

// Zero-input resource — was fetch_caches_for_requested_org(ctx) with no caching
ctx.fetch_cached(fetch_caches_for_requested_org, ()).await

// Multi-field input — use a tuple
ctx.fetch_cached(some_resource, (org_id, cache_id)).await
```

---

## Properties

| Property | Behaviour |
|---|---|
| **Cache key** | `(TypeId of fn item, input value)` |
| **Return type** | Fully generic — `Result`, `Option`, plain value, anything |
| **Caching behaviour** | Output is always cached unconditionally; no special-casing of errors |
| **Adding a new cached resource** | Write the `async fn`, call via `fetch_cached` — no other changes |
| **Zero-input resources** | `I = ()` — one cache slot per function, which is correct |
| **Multi-input resources** | `I = (A, B, ...)` — tuples are `Hash + Eq` with no extra work |
| **Auth variance** | `Ctx<Auth>` is part of the fn's type — a `RequireAuth` fn and a `MaybeAuth` fn never collide |
| **Closures** | Technically work but should be avoided — all instances of the same closure expression share one `TypeId`, so different captures would collide |
| **External deps** | None |