branch       ?=  main
cargo 		  = cargo
buf			  = buf

default: compile
compile: arwaind

install:
	@$(cargo) install --path ./crates/arwaind

.PHONY: arwaind
arwaind: proto
	@$(cargo) clippy -p arwaind
	@$(cargo) build -p arwaind

.PHONY: proto
proto:
	@buf generate -v api