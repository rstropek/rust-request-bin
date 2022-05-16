# ACR Beyond Images

## Introduction

This sample implements a simple "request bin". You can send HTTP GET and POST requests.
All requests are stored in-memory. You can view a list of all incoming requests in a simple
HTML UI.

## Installation

This project uses the following tools  development:

* [*just*](https://github.com/casey/just) to make it easier to run various development commands
* [*watchexec*](https://github.com/casey/just) to trigger *Cargo* builds whenever source (rs) changes

For that reason, you have to install the tools mentioned above:

* Install [*just*](https://github.com/casey/just): `cargo install just`
* Install [*watchexec*](https://github.com/watchexec/watchexec): `cargo install watchexec-cli`

## Routes

See [*requests.http*](requests.http) for supported routes and sample requests.
