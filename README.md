<!-- markdownlint-disable MD033 MD041 -->

<img width="64" height="64" align="left" style="float: left; margin: 20 10px 0 10;" src="icon.png">

# sudo

a convient (Windows) command-line utility for running processes as TrustedInstaller.

> [!IMPORTANT]
> TrustedInstaller is the equivalent of the superuser in Linux; THIS WILL GRANT MORE PRIVILEGES THAN [MICROSOFT'S SUDO](https://github.com/microsoft/sudo)

> [!NOTE]
> **THIS PROJECT DEPENDS ON [MinSudo](https://github.com/M2Team/NanaRun/releases)**. this project is literally a wrapper for it.

## How to

simply run `sudo` in any command prompt, let that be powershell or command prompt, and then it'll upgrade it to TrustedInstaller. (in some edge cases, it makes a new window. wtf?)

> [!WARNING]
> TrustedInstaller is extremely privilleged. Treat it as the superuser in Linux, as it really is.

~~Maybe at some point i'll add functionality to run other executables by sudo instead of just opening the prompt.~~

## Installation

copy both `MinSudo.exe` and `sudo.exe` to the `C:\Windows` or `C:\Windows\system32` folder. (or you can simply add it to the PATH)

## FAQ

> How does it work

I'm too lazy to actually make something work and run as TrustedInstaller, so this project depends on [MinSudo](https://github.com/M2Team/NanaRun/releases)

Don't worry, the 'installer' will download it for you.

> x64?

its compiled for x64

> x32?

okay, just compile for x32 bit and download the 32 bit of [MinSudo](https://github.com/M2Team/NanaRun/releases)

Maybe at some point i'll provide 32 bit binaries, but remember, 32 bit is a thing of the past!

> Linux?

**?????**

## Why?

simply type `sudo` and you're done, no fancy guis, no need to type out a long command.

## wtf is TrustedInstaller?

try to delete some random file in system32, say, for example, `hal.dll`, it will say something along the lines of `you need permission from TrustedInstaller to delete this file`

So in other words, TrustedInstaller is the "system account" with the highest privilleges, this is the equivalent of the `sudo` command in unix systems.

**tl;dr** TrustedInstaller is the superuser in Windows.
