.PHONY: install uninstall watcher install-dev publish

install:
	sh ./tools/install.sh

uninstall:
	sh ./tools/uninstall.sh

watcher:
	sh ./tools/watcher.sh

install-dev:
	sh ./tools/install-dev.sh

publish:
	sh ./tools/publish.sh
