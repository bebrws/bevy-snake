all: bevy-snake.js

bevy-snake.js: src/main.rs
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-name bevy-snake --out-dir ./ --target web target/wasm32-unknown-unknown/release/bevy-snake.wasm

http:
	python3 -m http.server