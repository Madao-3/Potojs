# 🦀 PotoJS

A Rust and WebAssembly project using
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

### Build

```
wasm-pack build
cd www
npm start
```
