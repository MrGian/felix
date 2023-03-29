<p align="center"><img src="https://user-images.githubusercontent.com/10211171/223741548-45ed1c96-a1da-40de-8544-8e10e4ddb072.png" height=100></p>
<p align="center"><sub><sup>Logo generated by Dall-E AI</sup></sub></p>
<h1 align="center">Felix OS</h1>
<h3 align="center">
(not yet) an x86 operating system.
</h3>

## Description

Felix is my attempt at writing an x86 operating system.

It's **written completely from scratch** in Rust and doesn't use any external dependecies.

![felix2](https://user-images.githubusercontent.com/10211171/223734499-15768aff-6d6f-4013-9fb5-3e75022a907e.png)<br>
*Felix running in Bochs emulator*

![output](https://user-images.githubusercontent.com/10211171/223737198-9aa156ca-1c57-4db5-932d-e999a1471dc0.gif)<br>
*Felix running on real hardware*

## Building

You can download a pre-built image or you can build it by yourself using Docker or the build script.

### Download pre-built image
[![build](https://github.com/mrgian/felix/actions/workflows/rust.yml/badge.svg)](https://github.com/mrgian/felix/actions)

A build is made for every commit.

To download the latest build click on the badge above, then click on the most recent build and download the artifact.

### Build using Docker
First make sure you have Docker installed. Then:

1. Clone the repo `git clone https://github.com/mrgian/felix`
2. Change dir to repo `cd felix`
3. Build the image `docker build -t felix-image .`
4. Run the container `docker run --name felix-container felix-image`
5. Copy build from container to host `docker cp felix-container:/root/felix/build build`

### Build using script
Make sure you have `Rust`,`mtools`,`dosfstools` and `fdisk` installed on your system. Then:

1. Clone the repo `git clone https://github.com/mrgian/felix`
2. Change dir to repo `cd felix`
3. Run build script `./build.sh`

## Running
The final disk image is `build/disk.img`

You can run it in QEMU using this command: `qemu-system-i386 -drive id=disk,file=build/disk.img,if=none,format=raw -device ahci,id=ahci -device ide-hd,drive=disk,bus=ahci.0`

Or you can run it on a real x86 computer by copying the disk image to a USB drive using this command: `sudo dd if=build/disk.img of=/dev/sdX status=progress` and then booting from USB.

## Progress
- *22/10/22* - Project start
- *27/01/23* - Bootloader can print to screen
- *31/01/23* - Bootloader can read data from disk to memory
- *01/02/23* - Bootloader can load kernel to memory
- *27/02/23* - Moved to Rust environment using inline assembly
- *01/03/23* - Rewritten kernel loading code in Rust
- *08/03/23* - Implemented println macro
- *20/03/23* - Switch to 32bit protected mode

## Credits
This project is entirely developed by **Gianmatteo Palmieri** ([mrgian](https://github.com/mrgian)).
