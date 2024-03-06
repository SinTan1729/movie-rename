PREFIX := /usr/local
PKGNAME := movie-rename

build:
	cargo build --release

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

aur: build
	tar --transform 's/.*\///g' -czf $(PKGNAME).tar.gz target/release/$(PKGNAME) target/autocomplete/* $(PKGNAME).1

.PHONY: build build-debug install clean uninstall aur
