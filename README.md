# rust-web-framework

Collection of utility functions used to build [Rust Web Server](https://github.com/bohdaq/rust-web-server). Can be useful while developing HTTP related functionality.

Core lib for [rust-web-server](https://github.com/bohdaq/rust-web-server), [rust-tls-server](https://github.com/bohdaq/rust-tls-server) and [http-to-https-letsencrypt](https://github.com/bohdaq/http-to-https-letsencrypt).

**NOTE! The corresponding crate is called [rust-web-server](https://crates.io/crates/rust-web-server)**.

## Features
1. [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS). Allowing resources to be used on other domains can be crucial for providing APIs and services. Knowing how cumberstone and difficult is the process to setup the CORS, server ships with CORS enabled to all requests by default.
1. [HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests). Server supports requests for the part of the file, or several different parts of the file.
1. [HTTP Client Hints](https://developer.mozilla.org/en-US/docs/Web/HTTP/Client_hints). Proactively asking client browser for suitable additional information about the system.
1. [X-Content-Type-Options](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options) set to nosniff, prevents from MIME type sniffing attacks.
1. [X-Frame-Options](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options). Site is not allowed to be embedded into iframe on other domains.
1. [Symlinks](https://en.wikipedia.org/wiki/Symbolic_link). You can have symlinks in your folder and they will be resolved correctly.
1. [Caching](https://developer.mozilla.org/en-US/docs/Web/HTTP/Caching#dealing_with_outdated_implementations) done right. It means no caching and therefore no outdated uncontrollable resources.
1. Resolving .html files without .html in path. It means if you try to open /some-html-file it will open file some-html-file.html and won't show 404 not found error. Same applies for folders. If you try to open /folder it will open file folder/index.html
1. Extensive logging. It means server prints the request-response pairs as they are so you can see all the details like request method, path, version and headers.
1. No third party dependencies.
1. [Forms](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form) (without files)

## Documentation
Open [documentation](src/README.md) for details.

## Community
Use GitHub [discussions](https://github.com/bohdaq/rust-web-framework/discussions), [issues](https://github.com/bohdaq/rust-web-framework/issues) and [pull requests](https://github.com/bohdaq/rust-web-framework/pulls).

There is Rust Web Server [Discord](https://discord.gg/zaErjtr5Dm) where you can ask questions and share ideas.

Follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Frequently Asked Questions
Open [FAQ](FAQ.md) for details.

## Donations
Send me tips via [PayPal](https://www.paypal.com/donate/?hosted_button_id=7J69SYZWSP6HJ).

