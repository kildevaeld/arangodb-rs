
.PHONY: all build

all: build

build: arangodb-http
	@cargo build

arangodb-http:
	@apigen gen -g Rust -o $(PWD)/$@ $(PWD)/api
	@sed -e 's#name = "package"#name = "arangodb-http"#' $(PWD)/arangodb-http/Cargo.toml > $(PWD)/arangodb-http/Cargo2.toml
	@mv $(PWD)/arangodb-http/Cargo2.toml $(PWD)/arangodb-http/Cargo.toml




clean:
	@rm -rf arangodb-http