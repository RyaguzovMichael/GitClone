all: build copy

build:
	cargo build --bin gclone --release

copy:
	copy ".\data\target.config" "C:Program Files\gclone\target.config"
	copy ".\data\target\release\gclone.exe" "C:\Program Files\gclone\gclone.exe"
