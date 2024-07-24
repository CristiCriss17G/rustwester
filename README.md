# rustwester

Simple rust cross platform executable to run a debugging web server

## Usage

```bash
rustwester --help
Usage: rustwester [OPTIONS]

Options:
  -b, --bind <BIND>       Host to listen to [env: BIND=] [default: 0.0.0.0]
  -p, --port <PORT>       Service port [env: PORT=] [default: 9999]
  -j, --json              Json response [env: JSON=]
  -v, --verbose...        Turn debugging information on repetitive use increases verbosity, at most 2 times
      --use-json-logging  Show logging information as json [env: USE_JSON_LOGGING=]
  -h, --help              Print help
  -V, --version           Print version
```

## Routes

- `/` - `GET` - Returns a simple hello world message
- `/echo` - `POST` - Returns the body of the request
- `/hey` - `GET` - Returns a simple hello there message
