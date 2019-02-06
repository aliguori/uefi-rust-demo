Prerequisites
-------------
You need a nightly rust toolchain installed (use rustup for that) and QEMU.

Install an OVMF Snapshot from Kraxel like:

https://www.kraxel.org/repos/jenkins/edk2/edk2.git-ovmf-x64-0-20190131.912.g3b6c73f13e.noarch.rpm

Building
--------

$ git submodule update --init
$ ./build.sh

Running
-------

$ ./run.sh

Break into the EFI menu by hitting escape.  Then execute:

Shell> \UEFI-APP.EFI

Credits
-------
This is just a version of Gil Mendes' blog post with the build.py file replaced
with simple shell scripts ported to the latest uefi-rs tree.  See:

https://medium.com/@gil0mendes/an-efi-app-a-bit-rusty-82c36b745f49