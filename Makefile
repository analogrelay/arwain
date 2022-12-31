branch       ?=  main
cargo 		  = cargo
buf			  = buf

default: compile install
all: compile install

compile: arwaind

.PHONY: arwaind
arwaind: proto
	@$(cargo) clippy -p arwaind
	@$(cargo) install --path ./crates/arwaind --debug --force

.PHONY: proto
proto:
	buf generate -v api