# HTTP server from scratch

A minimal HTTP/1.1 server implementation in Rust built to understand web fundamentals and Tokio async.

The goal is to be "unconditionally compliant" with [RFC 9110: HTTP Semantics](https://www.rfc-editor.org/rfc/rfc9110), [RFC 9111: HTTP Caching](https://www.rfc-editor.org/rfc/rfc9111.html), and [RFC 9112: HTTP/1.1](https://www.rfc-editor.org/rfc/rfc9112). This means implementing all features marked with "MUST", "MUST NOT", "REQUIRED", "SHALL", and "SHALL NOT", while ignoring "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL".

Here's an example to show how little you need to have a compliant server:

-   **GET and HEAD methods**: MUST be supported (RFC 9110)
-   **POST, PUT, DELETE, etc.**: are marked as OPTIONAL (RFC 9110)

## Lessons learned

-   `rayon` for CPU-bound computations and `tokio` for I/O bound computations.
-   tokio includes TCP, UDP, Unix sockets, timers, sync utilities, scheduler types, and other functions useful for async programming. I assumed it only included the async runtime
-   spawning a task submits it to the Tokio scheduler, which then delegates the task to the thread pool.
-   tasks require 64 bytes of memory and automatically move to free threads at awaits.
-   tasks are `Send` when all data that is held across `.await` calls is `Send`. (that's because Tokio might move the task to a different thread)
-   defaults to fast single threaded types, but the compiler forces thread-safe when crossing thread boundaries (delegating to thread pool)
-   reading a `TcpStream` requires `&mut` as we're modifying the internal state (`read_position`, `internal_buffers`)
-   RFC mentions no hard limit on request size just "Servers MUST be able to handle the URI of any resource they serve, and SHOULD be able to handle URIs of unbounded length if they provide GET-based forms that could generate such URIs" As a general purpose HTTP-server we'll follow the "MUST" and chunk data.
-   CR: carriage return, LF: line feed, SP: space character
-   HTTP/1.1 message structure is simple: `start-line CRLF *( field-line CRLF ) CRLF [ message-body ]`
-   There's no list of MUST/REQUIRED/SHALL status codes, but some are mentioned as side effects (like 501 being REQUIRED for unsupported methods). I included comments in my status code ENUM explaining why each one is implemented. Most of requirements for them follow the structure "If you reject a request for X, THEN you must use status code Y"

## Compromises made

This started as a pure RFC experiment but made minimal compromises for a working demonstration:

-   200 OK: Not explicitly REQUIRED but essential for successful GET/HEAD responses
-   404 Not Found: Not required, but needed to indicate missing resources
-   500 Internal Server Error: Not required, but needed for server errors

## Future deep dives inspired by this project

-   Implement epoll manually to create an async compatible feature (like an async socket listener from scratch)
