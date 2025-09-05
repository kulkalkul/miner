## Requirements

- Rust 1.90.0-nightly or newer

  - Install rustup via https://www.rust-lang.org/tools/install
    (or if you have other means of distribution via that)

  - Install nightly toolchain:
    ``rustup toolchain install nightly``

## Compilation
For debug:
``cargo build`` or ``cargo run``

For release:
``cargo build --release`` or ``cargo run --release``

Output is at ./target/debug ./target/release respectively

## Web Compilation
For debug:
``cargo build --target wasm32-unknown-unknown``

For release:
``cargo build --release --target wasm32-unknown-unknown``

Output is at ./target/wasm32-unknown-unknown/debug ./target/wasm32-unknown-unknown/release respectively

## How to embed into a webpage example

- Have an HTML page like so:
  ```html
  <!DOCTYPE html>

  <head>
  </head>

  <body>
  	<canvas id="glcanvas" tabindex="1">
  	</canvas>

  	<style>
  		html, body {
  		  width: 100%;
  		  height: 100%;
  		}

  		body {
  		  margin: 0px;
  		  background: #fafafa;
  		  display: flex;
  		  flex-direction: row;
  		}

  		canvas {
  		  margin: 0;
  		  padding: 0px;
  		  width: 640px;
  		  height: 480px;
  		  position: absolute;
  		  background: black;
  		  z-index: 0;
  		}
  	</style>

  	<script>
  		window.kulkalkul_asset_path = window.location.href;
  	</script>

  	<script src="./mq_js_bundle.js">
  	</script>

  	<script>load("./miner.wasm");
  	</script>

  </body>

  </html>
  ```

- Provide wasm file containing the game and embedded assets at path provided in the HTML file.
- Provide adjusted mq_js_bundle.js (which can be found in patched_js) file at path provided in the HTML file.
