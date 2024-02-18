# Snake Made Using Bevy!

Just as an experiment and reason to get some more Rust practice in..

## Faster Compiling and Building Statically

Remember to switch these two lines when not developing so you don't rely on dynamic linking:
```toml
# bevy = "0.13"
bevy = { version = "0.13.0", features = ["dynamic_linking"] }
```

## Helpful Info

The Dependency Injection system Bevy created it amazing! As seen with the setup functions..
Someone pointed me to documentation here: https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/chapter1/system.html

There is an unofficial book on Bevy here: https://bevy-cheatbook.github.io/programming/intro-data.html

## WASM Building

Requires 
```
cargo install -f wasm-bindgen-cli
```

Then run:
```
make
```

You will need html like so to run:
```
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <style>
      body {
        background: linear-gradient(
          135deg,
          white 0%,
          white 49%,
          black 49%,
          black 51%,
          white 51%,
          white 100%
        ) repeat;
        background-size: 20px 20px;
      }
      canvas {
        width: 100%;
        height: 100%;
        background-color: white;
      }
    </style>
    <title>Snake Written in Rust Using the Bevy Library</title>
  </head>
  <script type="module">
    import init from './bevy-snake.js'
    init()
  </script>
</html>

```