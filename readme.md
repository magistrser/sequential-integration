## Sequential integration
___
### Description

Lightweight library for sequential integration.
___
### Now support

Single, double and triple integrals with Simpson quadrature.
___
### Examples
```rust
sequential_integration::calculate_single_integral_simpson(
        "max(sqrt(1 - x^2))",    // equation
        -1,     // first_integral_begin
        1,      // first_integral_end
        0.01,   // first_integral_step
    )?
```

```rust
sequential_integration::calculate_double_integral_simpson(
        "1",    // equation
        -1,     // first_integral_begin
        1,      // first_integral_end
        0.01,   // first_integral_step
        "0",                    // second_integral_begin
        "max(sqrt(1 - x^2))",   // second_integral_end
        0.01,                   // second_integral_step
    )?
```

```rust
sequential_integration::calculate_triple_integral_simpson(
        "1",    // equation
        -1,     // first_integral_begin
        1,      // first_integral_end
        0.01,   // first_integral_step
        "0",                    // second_integral_begin
        "max(sqrt(1 - x^2))",   // second_integral_end
        0.01,                   // second_integral_step
        "0",                            // third_integral_begin
        "max(sqrt(1 - x^2 - y^2))",     // third_integral_end
        0.01,                           // third_integral_step
    )?
```

**equation** - _f(x)_ for single integral, _f(x,y)_ for double integral and _f(x,y,z)_ for triple integral

**first_integral_[begin/end]** - _constant_

**second_integral_[begin/end]** - _f(x)_

**third_integral_[begin/end]** - _f(x,y)_

Rules for writing equations: [mexprp](https://docs.rs/mexprp/0.3.0/mexprp/)