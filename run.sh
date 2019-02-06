qemu-system-x86_64 \
    -nodefaults \
    -vga std \
    -machine q35,accel=kvm \
    -m 128M \
    -drive if=pflash,format=raw,readonly,file=/usr/share/edk2.git/ovmf-x64/OVMF-pure-efi.fd \
    -drive if=pflash,format=raw,snapshot=on,file=/usr/share/edk2.git/ovmf-x64/OVMF_VARS-pure-efi.fd \
    -drive format=raw,file=fat:rw:target/x86_64-none-efi/debug/ \
    -serial stdio \
    -vga none
