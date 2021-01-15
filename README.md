<div align="center">
  👩🏽‍💻🚗💨
</div>

<h1 align="center">
  nixy
</h1>

<p align="center">
    a little boost where it matters thanks to <a href="https://nixos.org/download.html">nix</a>.
</p>

<div align="center">
  <a alt="crates.io" href="https://crates.io/crates/nixy">
    <img src="https://img.shields.io/crates/v/nixy.svg?logo=rust"/>
  </a>
  <a alt="license" href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-brightgreen.svg"/>
  </a>
</div>

<br />

This project is *alpha*, that means it probably doesn't do what it says on the tin, so contributions are definitely welcome!

## Inspiration

The idea came from [Burke's](https://www.youtube.com/watch?v=KaIRpx11qrc) tool used at Shopify. I couldn't find anything similar so decided to start this project. There is talk about [runix](https://discourse.nixos.org/t/remote-help-shopify-rebuild-our-world-in-nix/7571) being made public and there's another project scaffolding type [tool](https://github.com/hercules-ci/project.nix). In the [video](https://youtu.be/KaIRpx11qrc?t=2165) Burke explains how this provided an 80% speed up for developers getting started.

At the very least this is an exercise to learn more about Rust and Nix!

## Goals

- Nix can be productive, but the learning curve is prohibitive, let's make it easier.
- Write what we want to develop with using yaml (or JSON or CSV or whatever you like to use), convert this to a nix expression.
- Make system calls and use nix-env for setting up different workflows to save us some time.

## Roadmap

- [ ] As David I want to define what I need for my project in a yaml file so that I can save time in the future when starting similar projects.
  - [ ] Add a workflow that initializes environment for using the tool
  - [ ] Generate a template example
  - [ ] Convert a yaml example to nixexpr
  - [ ] Update the environment based on the template using the system call

## Install

``` bash
cargo install nixy
```

## Publishing new versions

Before publishing, we can check what we will publish.

``` bash
$ cargo publish --dry-run
```

and specifically what files will be sent

``` bash
$ cargo package --list
```


David MacEachern (davidmaceachern) 2021