# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

override IMAGE_NAME := MycrOS

override RUST_TARGET := x86_64-unknown-mycros.json
override RUST_PROFILE := dev

QEMU_ARGS := \
	-M q35 \
	-m 1G \
	-smp 4 \
	-debugcon stdio \
	-gdb tcp::1234 \
	-no-reboot \
	-no-shutdown

RUST_ARGS := \
	--manifest-path=kernel/Cargo.toml \
	--target $(RUST_TARGET) \
	--profile $(RUST_PROFILE) \
	-Zbuild-std=core,compiler_builtins,alloc \
	-Zbuild-std-features=compiler-builtins-mem \


ifeq ($(KVM),1)
	QEMU_ARGS += -enable-kvm
endif
ifeq ($(UEFI),1)
	QEMU_ARGS += -bios ovmf/OVMF.fd
run: ovmf
run-hdd: ovmf
endif

override RUST_TARGET_SUBDIR := $(basename $(RUST_TARGET))
override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

.PHONY: run
run: $(IMAGE_NAME).iso
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d

.PHONY: debug
debug: $(IMAGE_NAME).iso
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d -S

.PHONY: run-hdd
run-hdd: $(IMAGE_NAME).hdd
	qemu-system-x86_64 $(QEMU_ARGS) -hda $(IMAGE_NAME).hdd

ovmf:
	mkdir -p ovmf
	cd ovmf && curl -Lo OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

limine/limine:
	rm -rf limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v7.x-binary --depth=1
	$(MAKE) -C limine

.PHONY: kernel
kernel:
	cargo build $(RUST_ARGS)

.fsroot: kernel
	rm -rf .fsroot
	mkdir -p .fsroot/boot
	mkdir -p .fsroot/EFI/BOOT
	mkdir -p .fsroot/sys

	cp limine.cfg limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin .fsroot/boot/
	cp -v limine/BOOTX64.EFI limine/BOOTIA32.EFI .fsroot/EFI/BOOT/

	cp kernel/target/$(RUST_TARGET_SUBDIR)/$(RUST_PROFILE_SUBDIR)/kernel .fsroot/sys/

$(IMAGE_NAME).iso: limine/limine .fsroot
	xorriso -as mkisofs -b boot/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		.fsroot -o $(IMAGE_NAME).iso
	./limine/limine bios-install $(IMAGE_NAME).iso

$(IMAGE_NAME).hdd: limine/limine kernel
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd
	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00
	./limine/limine bios-install $(IMAGE_NAME).hdd
	mformat -i $(IMAGE_NAME).hdd@@1M
	mmd -i $(IMAGE_NAME).hdd@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M kernel/kernel ::/boot
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.cfg limine/limine-bios.sys ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTX64.EFI ::/EFI/BOOT
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTIA32.EFI ::/EFI/BOOT

.PHONY: clean
clean:
	rm -rf .fsroot $(IMAGE_NAME).iso $(IMAGE_NAME).hdd
	cargo clean  --manifest-path=kernel/Cargo.toml

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf
	cargo clean  --manifest-path=kernel/Cargo.toml
