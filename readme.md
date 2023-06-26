<div align="center">
  <img src="https://download.next-hat.com/ressources/images/logo.png" >
  <h1>vpnkitrc</h1>
  <p>

[![Stars](https://img.shields.io/github/stars/nxthat/vpnkitrc?label=%E2%AD%90%20stars%20%E2%AD%90)](https://github.com/nxthat/vpnkitrc)
[![Build With](https://img.shields.io/badge/built_with-Rust-dca282.svg?style=flat)](https://github.com/nxthat/vpnkitrc)
[![Chat on Discord](https://img.shields.io/discord/1011267493114949693?label=chat&logo=discord&style=flat)](https://discord.gg/WV4Aac8uZg)

  </p>

</div>

## ❓ Why Vpnkitrc

Vpnkitrc is a [`Rust`] client to interact with [`Vpnkit`] daemon usually running inside [`Docker Desktop`].

It have the ability to forward tcp and udp traffic and unix socket from the Docker Desktop VM to the Host.

I started this project to make [Nanocl](https://github.com/nxthat/nanocl) compatible with Docker Desktop users.

## 📙 Table of Contents

- [❓ Why Vpnkitrc ?](#-why-vpnkitrc)
- [📙 Table of Contents](#-table-of-contents)
- [📋 Requirements](#-requirements)
- [💾 Installation](#-installation)
- [📚 Documentation](#-documentation)

## 📋 Requirements

- [`Rust`] 1.70.0+
- [`Docker Desktop`]

## 💾 Installation

It's a [`Rust`] crates based on [`Ntex`] Http Client so you can install it with cargo and select the runtime eg:

```sh
cargo add vpnkitrc --features tokio
```

Available features are:

- `tokio`
- `glommio`
- `async-std`

## 📚 Documentation

- [`example`]
- [`docs.rs`]

[`Rust`]: https://www.rust-lang.org
[`Vpnkit`]: https://github.com/moby/vpnkit
[`Docker Desktop`]: https://www.docker.com/products/docker-desktop
[`Ntex`]: https://ntex.rs
[`docs.rs`]: https://docs.rs/vpnkitrc/0.1.0/vpnkitrc
[`example`]: ./example/
