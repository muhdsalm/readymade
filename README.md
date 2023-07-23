# Readymade
Install ready-made distribution images!

Readymade is a Linux Distribution installer based on the great [distinst](https://github.com/pop-os/distinst) library by System76.

It is created as a replacement to Red Hat's [Anaconda](https://github.com/rhinstaller/anaconda) installer for Ultramarine Linux and tauOS after we have heard many complaints about the poor UX design of Anaconda, and the lack of working alternative installers for RPM-based distributions.

# Why?

As we have mentioned previously, the reasons were:

- **Anaconda** is badly designed, bulky and has a very poor UX for both unattended installs (using Kickstart) and normal user installs. It is written in Python in an aging codebase, ported from Python 2 to 3, and has lots of legacy code from the 90s since the inception of the original Red Hat Linux. It also relies on the aging DNF 3 library which will be replaced by DNF 5 in the near future.
- **YaST** is designed for SUSE and SUSE only, it also suffers from the same issues as Anaconda, with an extremely large and complex codebase for simply being a wrapper for various system utilities.
- **Calamares**' support for Fedora is very hacky, as it is simply a barebones installer framework. It also has issues with BTRFS installs and does not support our desired features such as homed.

# Naming

As the convention of making up codenames for system components after J-Pop references, we have decided to name the installer after Ado's single, [Readymade](https://youtu.be/jg09lNupc1s) which happens to have a cool meaning to it as this installer essentially installs ready-made squashfs images.
