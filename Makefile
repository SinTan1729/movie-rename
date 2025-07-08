PREFIX := /usr/local
PKGNAME := movie-rename

build:
	cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34

build-debug:
	cargo build

clean:
	cargo clean

install: build
	install -Dm755 target/release/$(PKGNAME) "$(DESTDIR)$(PREFIX)/bin/$(PKGNAME)"
	install -Dm644 $(PKGNAME).1 "$(DESTDIR)$(PREFIX)/man/man1/$(PKGNAME).1"

uninstall:
	rm -f "$(DESTDIR)$(PREFIX)/bin/$(PKGNAME)"
	rm -f "$(DESTDIR)$(PREFIX)/man/man1/$(PKGNAME).1"

conf_tag := $(shell cat Cargo.toml | sed -rn 's/^version = "(.+)"$$/\1/p')
last_tag := $(shell git tag -l | tail -1)
bumped := $(shell git log -1 --pretty=%B | grep "build: Bumped version to " | wc -l)
tag:
ifneq (${conf_tag}, ${last_tag})
ifeq (${bumped}, 1)
	git tag ${conf_tag} -m "Version ${conf_tag}"
endif
endif

aur: build tag
	tar --transform 's/.*\///g' -czf $(PKGNAME).tar.gz target/x86_64-unknown-linux-gnu/release/$(PKGNAME) target/autocomplete/* $(PKGNAME).1

.PHONY: build build-debug install clean uninstall aur tag
