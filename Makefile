all: build copy

build:
	cargo build --bin gclone --release

copy:
	copy ".\target.config" "C:Program Files\gclone\target.config"
	copy ".\target\release\gclone.exe" "C:\Program Files\gclone\gclone.exe"
