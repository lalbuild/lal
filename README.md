# lal dependency manager [![build Status](https://engci-jenkins-gpk.cisco.com/jenkins/buildStatus/icon?job=team_CME/lal)](https://engci-jenkins-gpk.cisco.com/jenkins/job/team_CME/job/lal/)

A dependency manager built around artifactory and docker. See the [spec](./SPEC.md) for background information.

## Prerequisites
You need [docker](https://docs.docker.com/linux/step_one/) (minimum version 1.10), register an account with your username, then get someone to add the necessary credentials to your account.

You will need access to the [edonusdevelopers group](https://hub.docker.com/r/edonusdevelopers/), and you need to have called `docker login` on the command line as well.

## Installation
Two ways to install, depending on whether you can be bothered to run the rust install script:

### Precompiled releases (instant)
Fetch the static binaries compiled with [musl](http://www.musl-libc.org/) directly from [artifactory](https://engci-maven-master.cisco.com/artifactory/CME-release/lal/):

```sh
curl http://engci-maven.cisco.com/artifactory/CME-release/lal/latest/lal.tar | tar xz -C /usr/local
lal configure
```

Note that you will need to `sudo chown -R "$USER" /usr/local` to avoid using sudo on the tar side. Alternatively, install to another location and manage `$PATH` yourself.

When new versions are released, you will be told to re-run this command.

### From source (<10 minutes)
Get [stable rust](https://www.rust-lang.org/downloads.html) (inlined below), clone, build, install, and make it available:

```sh
curl -sSf https://static.rust-lang.org/rustup.sh | sh
git clone git@sqbu-github.cisco.com:Edonus/lal.git && cd lal
# install libssl-dev and curl (or distro equivalent) BEFORE you compile
cargo build --release
ln -sf $PWD/target/release/lal /usr/local/bin/lal
lal configure
```

When new versions are released, you will be told to `git pull && cargo build --release`.

## Usage
Illustrated via common workflow examples below:

### Install and Update
Installing pinned versions and building:

```sh
git clone git@sqbu-github.cisco.com:Edonus/media-engine
cd media-engine
lal fetch
# for canonical build
lal build
# for experimental
lal shell
docker> ./local_script
```

Updating dependencies:
(This example presumes ciscossl has independently been updated to version 6 and is ready to be used elsewhere.)

```sh
lal update ciscossl=6 --save
lal build # check it builds with new version
git commit manifest.json -m "updated ciscossl to version 6"
git push
```

### Reusing Builds
Using stashed dependencies:

```sh
git clone git@sqbu-github.cisco.com:Edonus/ciscossl
cd ciscossl
# edit
lal build
lal stash asan
cd ../media-engine
lal update ciscossl=asan # update named version (always from stash)
lal build
```

This workflow allows building multiple components simultaneously, and `lal status` provides safeguards and information on what dependencies you are using. Note that while doing this, you will receive warnings that you are using non-canonical dependencies.

### Creating a new version
Designed to be handled by jenkins on each push to master (ideally through validated merge). Jenkins should create your numeric tag and upload the build output to artifactory. This behaviour is handled in [jenkins-config](https://sqbu-github.cisco.com/Edonus/jenkins-config).

### Creating a new component
Create a git repo, `lal init` it, then update deps and verify it builds.

```sh
mkdir newcomponent
cd newcomponent
lal init # create manifest
git init
git remote add origin git@sqbu-github.cisco.com:Edonus/newcomponent.git
git add manifest.json
git commit -m "init newcomponent"
# add some dependencies to manifest
lal update gtest --save-dev
lal update libwebsockets --save
# create source and iterate until `lal build` passes

# later..
git commit -a -m "inital working version"
git push -u origin master
```

The last changeset will be tagged by jenkins if it succeeds. These have been done in two changesets here for clarity, but they could be done  in the same change.

Note that to set up jenkins jobs and commit hooks you need to follow usage instructions on [github-config](https://sqbu-github.cisco.com/Edonus/github-config#usage), and then [jenkins-config](https://sqbu-github.cisco.com/Edonus/jenkins-config#usage).

## Docker Image
The `build` and `shell` commands will use `docker run` on a configured image. For this to work without messing with permissions, two conditions must be met:

- configured docker image must have a `lal` user with uid `1000`
- linux user outside docker must be have uid `1000`

We have found this can be satisfied for most linux users and owned containers. The linux user restriction is unfortunately easier than to get docker usernamespaces working (which is currently incompatible with features like host networking).

## Developing
To hack on `lal`, follow normal install procedure, but build non-release builds iteratively.
When developing we do not do `--release`. Thus you should for convenience link `lal` via `ln -sf $PWD/target/debug/lal /usr/local/bin/lal`.

When making changes:

```sh
cargo build
lal subcommand ..args # check that your thing is good
cargo test # write tests
```

Before committing:

```sh
cargo fmt # requires `cargo install rustfmt` and $HOME/.cargo/bin on $PATH
```

## Autocomplete
Source the completion file in your `~/.bashrc` or `~/.bash_completion`:

```sh
echo "source /usr/local/share/lal/lal.complete.sh" >> ~/.bash_completion
```

If you are installing to a different path, or compiling yourself, set the path to where you have this file. E.g., if compiling:

```sh
echo "source $PWD/lal.complete.sh" >> ~/.bash_completion
```

from source directory.

## Logging
Configurable via flags before the subcommand:

```sh
lal fetch # normal output
lal -v fetch # debug output
lal -vv fetch # all output
```

### Influences
Terms used herein reference [so you want to write a package manager](https://medium.com/@sdboyer/so-you-want-to-write-a-package-manager-4ae9c17d9527#.rlvjqxc4r) (long read).
