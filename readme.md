## Sequential integration
___
### Description

Lightweight library for sequential integration.
___
### Now support

Single, double and triple integrals with Simpson quadrature.
___

### Examples version 1.\*.\*
```rust
sequential_integration::calculate_single_integral_simpson(
        |x: f64| (1. - x.powf(2.)).sqrt(),    // equation
        -1.,    // first_integral_begin
        1.,     // first_integral_end
        0.01,   // first_integral_step
    )?
```

```rust
sequential_integration::calculate_double_integral_simpson(
        |_x, _y| 1.,    // equation
        -1.,        // first_integral_begin
        1.,         // first_integral_end
        0.01,       // first_integral_step
        |_x| -0.,                               // second_integral_begin
        |x: f64| (1. - x.powf(2.)).sqrt(),      // second_integral_end
        0.01,                                   // second_integral_step
    )?
```

```rust
sequential_integration::calculate_triple_integral_simpson(
        |x: f64, y: f64, z: f64| x.powf(2.) + y.powf(2.) + z.powf(2.),    // equation
        -1.,    // first_integral_begin
        1.,     // first_integral_end
        0.01,   // first_integral_step
        |x| x,          // second_integral_begin
        |x| x / 2.,     // second_integral_end
        0.01,           // second_integral_step
        |x: f64, y: f64| x.powf(2.) + y,        // third_integral_begin
        |_x, _y| 0.,                            // third_integral_end
        0.01,                                   // third_integral_step
    )?
```

**equation** - _f(x)_ for single integral, _f(x,y)_ for double integral and _f(x,y,z)_ for triple integral<br/>
**first_integral_[begin/end]** - _constant_<br/>
**second_integral_[begin/end]** - _f(x)_<br/>
**third_integral_[begin/end]** - _f(x,y)_<br/>

___

### Release updates:
**0.0.1** - Double and triple integrals with Simpson quadrature <br/>
**0.0.2** - Not use additional memory, correct way for last step<br/>
**0.0.3** - Support single integral<br/>
**0.0.4** - Support integration from larger bound to smaller bound<br/>
**1.0.0** - Use closures instead string equations (See [mexprp](https://docs.rs/mexprp/0.3.0/mexprp/) if you want to use string equations with closures)<br/>

___
___

# Examples for old versions:

### Examples version 0.\*.\*
```rust
sequential_integration::calculate_single_integral_simpson(
        "max(sqrt(1 - x^2))",    // equation
        -1.,    // first_integral_begin
        1.,     // first_integral_end
        0.01,   // first_integral_step
    )?
```

```rust
sequential_integration::calculate_double_integral_simpson(
        "1",    // equation
        -1.,        // first_integral_begin
        1.,         // first_integral_end
        0.01,       // first_integral_step
        "0",                    // second_integral_begin
        "max(sqrt(1 - x^2))",   // second_integral_end
        0.01,                   // second_integral_step
    )?
```

```rust
sequential_integration::calculate_triple_integral_simpson(
        "x ^ 2 + y ^ 2 + z ^ 2",    // equation
        -1.,    // first_integral_begin
        1.,     // first_integral_end
        0.01,   // first_integral_step
        "x",        // second_integral_begin
        "x / 2",    // second_integral_end
        0.01,       // second_integral_step
        "x^2 + y",      // third_integral_begin
        "0",            // third_integral_end
        0.01,           // third_integral_step
    )?
```