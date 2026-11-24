
Gallium Cloud CLI
=================

The Gallium CLI tool allows you to SSH to virtual machines hosted on servers running
the Gallium cloud platform.

Usage
-----

First, login to your account with

    ga login

Then you can ssh to an instance based on its name with

    ga ssh gallium@my-instance

If the instance name is ambiguous, you can also use the instance ID:

    ga ssh gallium@7x0boKoZRB6ejT1773YtdwDY


Currently only virtual machines with a NAT network are supported, but in the future we will support any instance with an SSH server. 

Installing from source
----------------------

If you have a working Rust toolchain, you can install from source using

    cargo install --git https://github.com/gallium-cloud/gallium-cli
