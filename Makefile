name = prompt

.PHONY: build
build:
	cargo build

TARGETS = x86_64-unknown-linux-gnu x86_64-unknown-linux-musl x86_64-apple-darwin

.PHONY: release
release:
	mkdir -p release
	$(foreach TARGET,$(TARGETS), $(call buildrelease,$(TARGET)))

define buildrelease
rustup target add $(1)
cargo build --release --target $(1)
mv target/$(1)/release/$(name) $(name)-$(1)
strip $(name)-$(1)
endef

clean:
	rm -rf release target
