# Use PhantomData to Reduce Duplication

## Status

Accepted

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

By accepting this proposal, we reduce the maintenance burden, but are making the library more difficult to consume by a little bit.

This does indeed seem to be a zero cost abstraction.
The examples in [stm32f3-discovery](https://github.com/rubberduck203/stm32f3-discovery) were compiled in release mode and compared.

```sh
cargo build --release --examples
ls examples/ | sed s/.rs// | sed s,^,target/thumbv7em-none-eabihf/release/examples/, | xargs cargo size
```

### Old Version

```txt
   text	   data	    bss	    dec	    hex	filename
   4784	      0	      4	   4788	   12b4	target/thumbv7em-none-eabihf/release/examples/blinky
   4732	      0	      4	   4736	   1280	target/thumbv7em-none-eabihf/release/examples/button
   4564	      0	      4	   4568	   11d8	target/thumbv7em-none-eabihf/release/examples/button_int
   5460	      0	      4	   5464	   1558	target/thumbv7em-none-eabihf/release/examples/roulette
```

### New (PhantomData) Version

```txt
   text	   data	    bss	    dec	    hex	filename
   4784	      0	      4	   4788	   12b4	target/thumbv7em-none-eabihf/release/examples/blinky
   4732	      0	      4	   4736	   1280	target/thumbv7em-none-eabihf/release/examples/button
   4564	      0	      4	   4568	   11d8	target/thumbv7em-none-eabihf/release/examples/button_int
   5460	      0	      4	   5464	   1558	target/thumbv7em-none-eabihf/release/examples/roulette
```