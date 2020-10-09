# mobi2buy-api-tiny-url

## Installation

```bash
docker-compose up --remove-orphans

# or build

docker build -t mobi2buy/base -f Dockerfile_base .
docker build -t mobi2buy/api .

```

## Usage

### Set tiny url

```bash
 $ curl -d '{"url": "http://google.com", "tiny_url": "cap"}' -H "Content-Type: application/json" -X POST http://localhost:8001

 {"$oid":"5f80b0ff3239360100095051"}
```

### Redirect

```bash
$ curl  -I http://localhost:8001/cap

 HTTP/1.1 308 Permanent Redirect
Location: http://google.com
Server: Rocket
Content-Length: 0
Date: Fri, 09 Oct 2020 18:49:59 GMT
```

### Auto create tiny url

```bash
$ curl -d '{"url": "http://google.com"}' -H "Content-Type: application/json" -X POST http://localhost:8001

{"$oid":"5f80b0ff3239360100095051"}
```

### Delete older than 7 days

```bash
curl -H "Content-Type: application/json" -X DELETE http://localhost:8001/old

true
```

## Dev

### Build

```bash
echo -e "MONGO_ADDR=localhost
DB_NAME=db
MONGO_PORT=27017" > .env
```

```bash
rustup default nightly 
```

```bash
cargo run &
```

### Tests

```rust
cargo test -- --test-threads=1
```

#### Delete All - dev

```bash
curl -H "Content-Type: application/json" -X DELETE http://localhost:8001/

true
```