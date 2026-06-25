# HTTP Server From Scratch

A small HTTP server I built from scratch using TCP sockets as part of the CodeCrafters HTTP Server challenge.

The project handles basic HTTP/1.1 requests, parses headers and request bodies manually, serves files, supports concurrent connections, gzip compression, and persistent connections.

I built this to better understand what happens behind web frameworks, especially how requests, responses, headers, and TCP connections actually work at a lower level.

## What this project does

This server can handle basic HTTP requests and respond like a real web server.

Some of the features include:

- Binding to a TCP port
- Accepting incoming client connections
- Parsing HTTP request lines
- Reading HTTP headers
- Handling `GET` requests
- Handling `POST` requests
- Returning plain text responses
- Returning files from a directory
- Reading request bodies
- Supporting multiple concurrent connections
- Supporting gzip compression
- Handling persistent connections
- Closing connections based on HTTP headers

## Why I built this

I wanted to understand how HTTP works at a lower level.

Usually when building backend apps, a framework hides a lot of details. This project helped me understand what actually happens before the request reaches the application layer.

For example:

- How a TCP server listens for connections
- How HTTP messages are structured
- How headers are parsed
- How request bodies are read
- How files are served over HTTP
- How multiple clients can connect at the same time
- How compression is negotiated using headers
- How persistent connections work

## Features implemented

### Basic HTTP response

The server can return a simple `200 OK` response.

Example:

```http
HTTP/1.1 200 OK
```

### URL path handling

The server can extract the path from the request line.

For example:

```http
GET /hello HTTP/1.1
```

The server can read `/hello` and use it to decide what response to return.

### Response body

The server can return a body with the correct `Content-Length` and `Content-Type` headers.

Example:

```http
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 5

hello
```

### Header parsing

The server reads and parses request headers like:

```http
User-Agent: curl/7.64.1
Accept-Encoding: gzip
Connection: close
```

This is used for features like reading headers, compression, and connection handling.

### Concurrent connections

The server supports multiple clients connecting at the same time.

This was one of the more important parts because a real server should not block every other request just because one client is connected.

### File serving

The server can return files from a given directory.

Example request:

```http
GET /files/example.txt HTTP/1.1
```

If the file exists, the server returns the file content.

If the file does not exist, it returns:

```http
HTTP/1.1 404 Not Found
```

### POST request body

The server can read the request body from a `POST` request and write it into a file.

Example:

```http
POST /files/test.txt HTTP/1.1
Content-Length: 11

hello world
```

### Gzip compression

The server supports gzip compression when the client sends:

```http
Accept-Encoding: gzip
```

If gzip is supported, the response body is compressed and the server includes:

```http
Content-Encoding: gzip
```

### Persistent connections

The server supports persistent HTTP connections, meaning the same TCP connection can be used for more than one request.

It also handles connection closure when the client sends:

```http
Connection: close
```

## What I learned

This project helped me understand backend development from a lower level.

The most useful parts for me were:

- HTTP is basically text over TCP
- Headers matter a lot
- `Content-Length` is important for reading request bodies correctly
- A server needs to be careful about blocking connections
- File serving needs proper error handling
- Compression depends on what the client says it accepts
- Persistent connections make request parsing more complicated because one connection can contain multiple requests

## Tech used

- TCP sockets
- HTTP/1.1
- Multithreading / concurrency
- File I/O
- Gzip compression

## Why this matters

Even though this is a small project, it connects directly to backend engineering.

Most backend work happens at a higher level, but understanding HTTP, sockets, headers, request bodies, and connection handling makes debugging APIs and production systems easier.

This project gave me a clearer picture of what web frameworks are doing behind the scenes.
