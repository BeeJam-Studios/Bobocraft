ifeq ($(OS),Windows_NT)
	LN = mklink /D client\assets\gltf ..\..\..\robocraft-1\gltf
else
	LN = ln -s ../../../robocraft-1/gltf client/assets/gltf
endif


bobocraft: client/assets/gltf
	BEVY_ASSET_PATH="$(CURDIR)/client/assets" cargo build

client/assets/gltf:
	$(LN)

clean:
	cargo clean

.PHONY: bobocraft clean
