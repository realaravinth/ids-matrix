[![Build Status](https://travis-ci.com/realaravinth/ids-matrix.svg?branch=master)](https://travis-ci.com/realaravinth/ids-matrix)
[![License: GPL v2](https://img.shields.io/badge/License-GPL%20v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
[![dependency status](https://deps.rs/repo/github/realaravinth/ids-matrix/status.svg?style=flat-square)](https://deps.rs/repo/github/realaravinth/ids-matrix)

# ids-matrix
A lightweight Intrusion Detection System built on top of matrix
<br><br>
**WARNING: `realaravinth/ids-matrix` comes with ABSOLUTELY NO WARRANY, to the extent permitted by applicable law.**
<br><br>
The instructions provided here are for the Debian(Buster) GNU/Linux, however it can be modified to work with other distrubtions as well.
It uses `pam_exec` to collect login information so kindly refer to your distribution's guide if this guide doesn't work.

## Table of contents:

- [Dependencies:](#dependencies)
- [How to compile](#how-to-compile)
- [Installation](#installation)
- [Contributions](#contributions)

## Dependencies

  * libpam-modules
  * coreutils(uses `date` and `uname`)[optional]
  * net-tools(uses `hostname`)[optional]<br>
  * rust(to compile)
  *optional: you can remove parts of the code if you wish to not to install the optional dependencies*

## How to compile

  1. `cd` into source directory
  2. edit src/main.rs and fill
    * `server`: url of your matrix server
    * `access_token`: access token of your matrix account(create new one with least priveleges)
    * `room_id`: room ID of the room where you want to publish the updates
  3. `cargo build --release`
    This compiles the program and places the binary in `target/release` directory

## Installation
  
  1. Place `ids-matrix` binary in `/usr/local/bin` of your server
  2. Append the following to `/etc/pam.d/sshd`(always take backup of the original config):<br>
    `session optional pam_exec.so /usr/local/bin/ids-matrix`
    
## Contributions
Yes please! Fork this repo and send in PRs, I'll be happy to review and merge them!  
