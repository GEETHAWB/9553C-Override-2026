send:
	@VERSION=$$(($$(cat VERSION) + 1)); \
	echo $$VERSION > VERSION; \
	echo ">> Uploading version $$VERSION ..."; \
	cargo v5 upload --release

build:
	echo ">> Building ..."; \
	cargo build --package override --release