# lal
[![build status](https://github.com/lalbuild/lal/workflows/pull_request/badge.svg)](http://github.com/lalbuild/lal/actions)
[![coverage status](http://img.shields.io/coveralls/lalbuild/lal.svg)](https://coveralls.io/r/lalbuild/lal)
[![crates status](https://img.shields.io/crates/v/lal.svg)](https://crates.io/crates/lal)

A strict, language-agnostic build system and dependency manager.

* **Use existing tools**: `lal build` only shells out to an executable `BUILD` script in a configured docker container. Install what you want in your build environments: cmake, autotools, cargo, go, python.
* **Cache large builds**: publish built libraries for later use down the dependency tree.
* **Strict with environments and versions**: `lal verify` enforces that all your dependencies are built in the same environment and use the same version down the tree (and it runs before your build).
* **Builds on existing package manager ideas**: versions in a manifest, fetch dependencies first, verify them, then build however you want, lal autogenerates lockfiles during build.
* **Transparent use of docker for build environments** with configurable mounts and direct view of the docker run commands used. `lal shell` or `lal script` provides additional easy ways to use the build environments.

## Conception
We needed a simple dependency manager built around the idea of a storage backend and a build environment. Strict versioning and consistent build environments for our C++ codebases where the most important features needed, and we already had docker and artifactory for the rest, however other storage backends can be implemented in the future.

The command line [specification](./SPEC.md) contains a detailed overview of what `lal` does.

## Showcases
A few short ascii shorts about how lal is typically used internally:

- [build / fetch](https://asciinema.org/a/3udzvbettco6sx44mbn238x0v)
- [custom dependencies](https://asciinema.org/a/c9v790m4euh190ladaqzfdc43)
- [scripts](https://asciinema.org/a/a3xmki0iz5j0am2vv780p41xa)

## Setup
Needs a few pieces to be set up across a team at the moment. Grab a :coffee:

### Prerequisites (devs)
You need [docker](https://docs.docker.com/engine/installation/linux/) (minimum version 1.12), logged into the group with access to your docker images in the [relevant config file](./configs). Distros with Linux >= 4.4.0 is the primary target, but Mac is also getting there.

### Prerequisites (ops)
A set of docker images as outlined in the [relevant config file](./configs), all built to include a `lal` user and available to docker logged in devs (see below)

CI setup to build and upload releases of master as outlined further below.

A configured backend in same config file, distrubuted with lal to your devs. Currently, this only supports artifactory.

## Installation
If you do not want to install rust, get a statically linked version of lal:

```sh
curl -sSL https://github.com/lalbuild/lal/releases/download/v3.8.1/lal.tar.gz | sudo tar xz -C /usr/local
echo "source /usr/local/share/lal/lal.complete.sh" > ~/.bash_completion
curl -sSL https://raw.githubusercontent.com/lalbuild/lal/master/configs/demo.json > cfg.json
lal configure cfg.json
```

These are built on [CI](https://github.com/lalbuild/lal/releases) via [muslrust](https://github.com/clux/muslrust). You can drop `sudo` if you own or `chown` your install prefix.

## Building
Clone, install from source with [rust](https://www.rust-lang.org/en-US/install.html), setup autocomplete, and site configuration.

The following sections assume you are working from a local git clone of the repository.

```sh
git clone git@github.com:lalbuild/lal.git
cd lal
echo "source $PWD/lal.complete.sh" >> ~/.bash_completion
```

### Native (with cargo)

If you don't have `lal` installed yet, you can install it using `cargo install` from git.
It doesn't use `docker` if you don't have it and it doesn't depend on having `lal` installed.

However, this is not the cleanest build method, and may dynamically link against your system libraries.
The compiled binary may break with future system updates.

```sh
cargo install

# Set the initial configuration from defaults
lal configure
```

### Native (with lal)

If you already have `lal` installed, you can build `lal` with `lal`.

```sh
lal build
cp -v ./OUTPUT/bin/lal ~/bin/lal

# Pick up configuration updates by reconfiguring with the new binary
lal configure
```
This is our preferred way to update lal from source.
It doesn't use `docker` if you don't have it, as it uses your local rustup toolchains.
The compiled binary is statically linked, using our configuration options by default.

### Docker (with muslrust)

If you have `docker` and `lal` installed, but don't have a rust compiler, you can use our shared toolchain.

```sh
# Set a temporary override to use a prepared `rust` environment.
lal env set rust

lal build
cp -v ./OUTPUT/bin/lal ~/bin/lal
lal configure
```
The compiled binary is statically linked, using our build toolchain for reproducibility.

## Usage

### Creating a new component
Create a git repo, lal init it, then update deps and verify it builds.

```sh
lal init alpine # create manifest for a alpine component
git add .lal/
git commit -m "init newcomponent"
# add some dependencies to manifest (if you have a storage backend)
lal update gtest --save-dev
lal update libwebsockets --save
# create source and iterate until `lal build` passes

# later..
git commit -a -m "inital working version"
git push -u origin master
```

Note that the first `lal build` will call `lal env update` to make sure you have the build environment.

### Creating a new version
Designed to be handled by CI on each push to master (ideally through validated merge). CI should create your numeric tag and upload the build output to artifactory.  See the [spec](./SPEC.md) for full info.

## Docker Image
The `build` and `shell` commands will use `docker run` on a configured image. The only condition we require of docker images is that they have a `lal` user added.

Normally, this is sufficient in a docker image to satisfy constraints:

```
RUN useradd -ms /bin/bash lal -G sudo && \
    echo "%sudo ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

VOLUME ["/home/lal/volume"]
```

Note that `sudo` is not necessary, but sometimes convenient.

We will use this user inside the container to run build scripts. By default this works best if the `id` of the host user is 1000:1000, but if it is not, then lal will create a slightly modified version of the image that matches the user id and group id for your host system.

This is a one time operation, and it is a more general solution for use than docker usernamespaces (which is currently incompatible with features like host networking).

## Developing
Have the [rust documentation for lal](https://cisco.github.io/lal-build-manager) ready.

To hack on `lal`, follow normal install procedure, but build non-release builds iteratively.
When developing we do not do `--release`. Thus you should for convenience link `lal` via `ln -sf $PWD/target/debug/lal /usr/local/bin/lal`.

When making changes:

```sh
cargo build
lal subcommand ..args # check that your thing is good
cargo test # write tests
```

Good practices before comitting (not mandatory):

```sh
cargo fmt # requires `cargo install rustfmt` and $HOME/.cargo/bin on $PATH
rustup run nighthly cargo clippy # requires nightly install of clippy
```

Note that if you have a rust environment set up in your lal config, you can actually `lal build lal` (which will use the provided `manifest.json` and `BUILD` file).

## Build issues
If libraries cannot be built, then upgrade `rustc` by running `rustup update stable`.

- missing ssl: install distro equivalent of `libssl-dev` then `cargo clean`
- fatal error: 'openssl/hmac.h' file not found If you are on a GNU/Linux distribution (like Ubuntu), please install `libssl-dev`. If you are on OSX, please install openssl and check your OpenSSL configuration:

```sh
brew install openssl
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
export DEP_OPENSSL_INCLUDE=`brew --prefix openssl`/include
```

## Runtime issues
### SSL Certificates
The lookup of SSL certificates to do peer verification can fail if they are missing or in a non-standard location. The search is done via the [openssl-probe crate](https://github.com/alexcrichton/openssl-probe/blob/master/src/lib.rs).

Although this shouldn't be necessary anymore; you can also override the search yourself by pointing to the certificates explicitly:

```
# OSX
export SSL_CERT_FILE=/usr/local/etc/openssl/cert.pem
# CentOS
export SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt
```


This should be put in your `~/.bashrc` or `~/.bash_profile` as `lal` reads it on every run. Note that the normal location is `/etc/ssl/certs/ca-certificates.crt` for most modern linux distros.

### Docker permission denieds
You need to have performed `docker login`, and your user must have been added to the correct group on dockerhub by someone in charge before you can pull build environments.

## Logging
Configurable via flags before the subcommand:

```sh
lal fetch # normal output
lal -v fetch # debug output
lal -vv fetch # all output
```

### Influences
Main inspirations were [cargo](https://github.com/rust-lang/cargo) and [npm](https://github.com/npm/npm).
A useful reference for the terms used throughout: [so you want to write a package manager](https://medium.com/@sdboyer/so-you-want-to-write-a-package-manager-4ae9c17d9527#.rlvjqxc4r) (long read).
