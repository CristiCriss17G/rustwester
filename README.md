# rustwester

Simple rust cross platform executable to run a debugging web server

> [!WARNING]
>
> Starting with version 3.0.0, support for Rust 1.75 and Windows 7 has been dropped.
>
> Previous versions are still available for older systems.

## Usage

```bash
rustwester --help
Usage: rustwester [OPTIONS]

Options:
  -b, --bind <BIND>          Host to listen to [env: BIND=] [default: 0.0.0.0]
  -p, --port <PORT>          Service port [env: PORT=] [default: 9999]
  -j, --no-json              Don't allow json response [env: NO_JSON=]
  -v, --verbose...           Turn debugging information on repetitive use increases verbosity, at most 2 times
      --use-json-logging     Show logging information as json [env: USE_JSON_LOGGING=]
      --log-file <LOG_FILE>  Log file location [env: LOG_FILE=]
  -h, --help                 Print help
  -V, --version              Print version
```

## Routes

- `/` - `GET` - Returns a simple hello world message
- `/echo` - `POST` - Returns the body of the request
- `/hey` - `GET` - Returns a simple hello there message

## Query Parameters

When the `--no-json` flag is not set, the following query parameters are available:

- `?json` - Returns a json response, for all routes, except for `/hey?json`

The executable will also respect `Accept` headers, if the `Accept` header is set to `application/json`, the response will be in json format.
