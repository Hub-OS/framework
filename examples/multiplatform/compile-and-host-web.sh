# Really simple script for testing
# Note that it will host every file in this folder, including the src directory

# installing wasm-pack:
# https://github.com/drager/wasm-pack
# https://drager.github.io/wasm-pack/installer/

# installing http-server:
# cargo install http-server

reset; wasm-pack build --target web --release && http-server -i
