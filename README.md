# Simple Rust Webserver

A multithreaded HTTP webserver built in Rust, developed as the final project from [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html) book.

## Overview

This project implements a simple HTTP server with a custom `ThreadPool` to handle concurrent requests. It serves as a practical synthesis of core Rust concepts including ownership, concurrency, and the standard library's threading and channel primitives.

## Features

- Custom `ThreadPool` implementation using worker threads and `mpsc` channels
- Concurrent request handling across multiple threads
- Basic HTTP routing with static file responses
