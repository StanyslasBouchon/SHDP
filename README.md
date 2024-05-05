# SHDP
Tends for Streamlined Hyper Data Protocol. It allows sending and receiving data in a compressed-way, especially when sending files.

Basic implementations are onto 2 (two) websites:
* [Botlyz](https://botlyz.com)
* [DIBI-LEKSIRO](http://dibi-leksiro.fr)

Those websites use SHDP to retrieve HTML, CSS or JS content dynamically while the user is browsing the website.

> [!NOTE]
> SHDP will not try to replace HTTP not TCP protocols. It is just a way to send data in a compressed way.

> [!IMPORTANT]
> SHDP is not standardized yet. It is still in development.

## How it works
See the [SHDP Protocol](PROTOCOL.md) page to understand the binary structure of the protocol.

## Implementations
* [Rust](RUST_IMPL.md) - The first implementation of SHDP is in Rust. It is a library that can be used in any Rust project.

## License
SHDP is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

