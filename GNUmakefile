default:
	git update-index --assume-unchanged hello-rust/libs/librust.a
	git update-index --assume-unchanged hello-rust/libs/librust-sim.a
	./build || rustup component add rust-src --toolchain nightly-x86_64-apple-darwin

list-devices:
	xcrun xctrace list devices
