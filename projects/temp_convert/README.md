# temp_convert

A simple command-line tool to convert temperatures between Celsius, Fahrenheit, and Kelvin.

## Usage

```bash
temp_convert <value> <from_unit> <to_unit>
```

*   `value`: The temperature value to convert (e.g., 25.5).
*   `from_unit`: The original unit of the temperature (C, F, or K).
*   `to_unit`: The desired unit of the temperature (C, F, or K).

## Examples

```bash
temp_convert 25 C F
```

This will convert 25 degrees Celsius to Fahrenheit.

```bash
temp_convert 298.15 K C
```

This will convert 298.15 Kelvin to Celsius.

## Supported Units

*   `C` or `c`: Celsius
*   `F` or `f`: Fahrenheit
*   `K` or `k`: Kelvin
