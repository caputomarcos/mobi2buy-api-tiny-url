# mobi2buy-api-tiny-url

## Installation

```bash
docker-compose up --remove-orphans
```

## Build Locally

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

## Usage

### Set tiny url

```bash
 curl -d '{"url": "http://google.com", "tiny_url": "cap"}' -H "Content-Type: application/json" -X POST http://localhost:8001
```

### Auto create tiny url

```bash
curl -d '{"url": "http://google.com"}' -H "Content-Type: application/json" -X POST http://localhost:8001
```

### Delete All - dev

```bash
curl -H "Content-Type: application/json" -X DELETE http://localhost:8001/old
```

### Delete All - dev

```bash
curl -H "Content-Type: application/json" -X DELETE http://localhost:8001/
```


## Tests

```rust
cargo test -- --test-threads=1
```