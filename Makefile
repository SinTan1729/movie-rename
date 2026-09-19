PREFIX := /usr/local
PKGNAME := movie-rename

build:
	cargo build --release --target x86_64-unknown-linux-musl

build-debug:
	cargo build

clean:
	cargo clean
	rm -f *.tar.gz

install: build
	install -Dm755 target/release/$(PKGNAME) "$(DESTDIR)$(PREFIX)/bin/$(PKGNAME)"
	install -Dm644 $(PKGNAME).1 "$(DESTDIR)$(PREFIX)/man/man1/$(PKGNAME).1"

uninstall:
	rm -f "$(DESTDIR)$(PREFIX)/bin/$(PKGNAME)"
	rm -f "$(DESTDIR)$(PREFIX)/man/man1/$(PKGNAME).1"

conf_tag := $(shell cat Cargo.toml | sed -rn 's/^version = "(.+)"$$/\1/p')
last_tag := $(shell git describe --tags --abbrev=0)
bumped := $(shell git log -1 --pretty=%B | grep "build: Bumped version to " | wc -l)
tag:
ifneq (${conf_tag}, ${last_tag})
ifeq (${bumped}, 1)
	git tag ${conf_tag} -m "Version ${conf_tag}"
endif
endif

aur: build
	tar --transform 's/.*\///g' -czf $(PKGNAME).tar.gz target/x86_64-unknown-linux-musl/release/$(PKGNAME) target/autocomplete/* target/man/*

release: aur
	gh release create "${last_tag}" --notes "$$(git-cliff --latest)" "$(PKGNAME).tar.gz"
	$(MAKE) clean

.PHONY: build build-debug install clean uninstall aur tag
