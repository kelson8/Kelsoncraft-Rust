VERSION=1.0
NAME=misc-test
EXEC=misc-test
PREFIX=$(HOME)/.local

#default: build_release
default: build_debug

# Taken from here
# https://github.com/BlockListed/rust-makefile/blob/main/Makefile

# This is an easy way to build this and the docs on Linux.
# Just run one of these commands:
# make clean - Clean the build directory.
# make build_release - Build release
# make build_debug - Build debug
# make doc - Generate documentation for project.

clean:
	@echo "Cleaning build dir"
	@rm -rf target/*
	@echo "Cleaning using cargo"
	@cargo clean

check:
	@echo "Checking $(NAME)"
	@cargo check

build_release:
	@echo "Building release: $(VERSION)"
	@cargo build --release

build_debug:
	@echo "Building debug"
	@cargo build
run:
	@echo "Running debug"
	@cargo run

doc:
	@echo "Generating Rust Documentation"
	@cargo doc

fix:
	@echo "Running cargo fix for fixing warnings."
	@cargo fix

#install_debug: build_debug
	@#echo "Installing debug"
	@#cp target/debug/$(EXEC) $(PREFIX)/bin

#install: build_release
	@#echo "Installing release: $(VERSION)"
	@#cp target/release/$(EXEC) $(PREFIX)/bin

# This could be used for copying this to the install directory such as ~/.local/bin
#dist: build_release
#	@if [ ! -d ./pkg ]; \
#	then \
#		mkdir ./pkg; \
#	fi
#
#	@if [ -d ./pkg/$(NAME)-$(VERSION) ]; \
#	then \
#		echo "Current version number already exists! Removing old files!"; \
#		rm -rf ./pkg/$(NAME)-$(VERSION); \
#	fi
#
#	@mkdir ./pkg/$(NAME)-$(VERSION)
#
#	@cp ./dist-scripts/install.sh ./pkg/$(NAME)-$(VERSION)/
#
#	@sed -i 's#{prefix}#$(PREFIX)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
#	@sed -i 's#{version}#$(VERSION)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
#	@sed -i 's#{name}#$(NAME)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
#	@sed -i 's#{exec}#$(EXEC)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
#
#	@mkdir ./pkg/$(NAME)-$(VERSION)/files
#	@cp target/release/$(EXEC) ./pkg/$(NAME)-$(VERSION)/files/
#	@strip ./pkg/$(NAME)-$(VERSION)/files/$(EXEC)
#
#	@cp LICENSE ./pkg/$(NAME)-$(VERSION)/
#
#	@cd ./pkg && tar -czf ./$(NAME)-$(VERSION).tar.gz ./$(NAME)-$(VERSION)
#	@echo "Cleaning up"
#	@rm -rf ./pkg/$(NAME)-$(VERSION)
