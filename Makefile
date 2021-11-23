CC=cargo
VERSION=1.0.0
NAME=encodedir
EXEC=encodedir
PREFIX=$(HOME)/.local

default: build_release
clean:
	@echo "Cleaning build dir"
	@rm -rf target/*

build_release:
	@echo "Building release: $(VERSION)"
	@cargo build --release
build_debug:
	@echo "Building debug"
	@cargo build
install_debug: build_debug
	@echo "Installing debug"
	@cp target/debug/$(EXEC) $(PREFIX)/bin
install: build_release
	@echo "Installing release: $(VERSION)"
	@cp target/release/$(EXEC) $(PREFIX)/bin
dist: build_release
	@if [ ! -d ./pkg ]; \
	then \
		mkdir ./pkg; \
	fi

	@if [ -d ./pkg/$(NAME)-$(VERSION) ]; \
	then \
		echo "Current version number already exists! Removing old files!"; \
		rm -rf ./pkg/$(NAME)-$(VERSION); \
	fi

	@mkdir ./pkg/$(NAME)-$(VERSION)

	@cp ./dist-scripts/install.sh ./pkg/$(NAME)-$(VERSION)/

	@sed -i 's#{prefix}#$(PREFIX)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
	@sed -i 's#{version}#$(VERSION)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
	@sed -i 's#{name}#$(NAME)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
	@sed -i 's#{exec}#$(EXEC)#g' ./pkg/$(NAME)-$(VERSION)/install.sh

	@mkdir ./pkg/$(NAME)-$(VERSION)/files
	@cp target/release/$(EXEC) ./pkg/$(NAME)-$(VERSION)/files/

	@tar -czf ./pkg/$(NAME)-$(VERSION).tar.gz ./pkg/$(NAME)-$(VERSION)
	@echo "Cleaning up"
	@rm -rf ./pkg/$(NAME)-$(VERSION)