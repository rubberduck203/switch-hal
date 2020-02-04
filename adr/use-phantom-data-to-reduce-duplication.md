# Use PhantomData to Reduce Duplication

## Status

Pending verification that the generic `PhantomData` version is indeed zero cost.

## Context

The current implementation requires identical structs to hold the `OutputPin` for `ActiveHighOutputSwitch` and `ActiveLowOutputSwitch`.  
This also means there are identical implementations of `new()`, `into_pin()`, `ToggleableOutputSwitch`.

## Decision

This could be cleaned up by using a single struct, with a second type parameter.

```rust
pub struct ActiveHigh;
pub struct ActiveLow;

pub struct Switch<T, Activeness>
where
    T: OutputPin,
{
    pin: T,
    active: PhantomData<Activeness>,
}
```

This is a zero-cost abstraction, because `PhanotomData<T>` doesn't allocate any memory.  
However, this means that construction of an `OutputSwitch` becomes more complicated for the library consumer.

Before the change:

```rust
let led = ActiveHighOutputSwitch::new(
    gpioe
    .pe9
    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
);
```

After the change:

```rust
let led = output::Switch::<_, ActiveHigh>::new(
    gpioe
    .pe9
    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
);
```

## Consequences

By rejecting this proposal, we are adding more (and duplicated!) code to the library, increasing maintenance burden.

By accepting this proposal, we reduce the maintenance burden, but are making the library more difficult to consume by a small bit.