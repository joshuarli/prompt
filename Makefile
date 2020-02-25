name = prompt

target/debug/$(name): src/main.rs
	cargo build

test: target/debug/$(name)
	./test.sh

TARGETS = x86_64-unknown-linux-gnu x86_64-unknown-linux-musl x86_64-apple-darwin

.PHONY: release
release:
	mkdir -p release
	$(foreach TARGET,$(TARGETS), $(call buildrelease,$(TARGET)))

define buildrelease
rustup target add $(1)
cargo build --release --target $(1)
mv target/$(1)/release/$(name) release/$(name)-$(1)
# dunno why "rustup target add ...musl" is appended to strip args
# strip release/$(name)-$(1)
endef

clean:
	rm -rf release target
