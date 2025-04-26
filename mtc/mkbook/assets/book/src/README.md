# Pay2Vault

some cool description


## DEV

#### INSTALL

1. install [just](https://just.systems/man/en/)

2. install prototools

```
brew install protobuf
```

3. install dev tools

```
just dev-install
```

or use tui :

```
just
```

and choose dev-install

#### USAGE

```
just
```

## CONFIG

in file:
`config/server_config.toml`

## RUN 

- direct

```bash
just run
```

- docker 
    -   prefered - build speed should be much faster
    -   no install step needed

```
docker compose up
```

if port was changed in `config/server_config.toml` it need to be also changed in `docker-compose.yml` 

## CRATES

### crates/[server](./crates/server/README.md)

main crate with http/grpc server

### crates/[config](./crates/config/README.md)

config lib crate to handle config save/load for any other crate

### crates/[config-derive](./crates/config-derive/README.md)

internal crate with derive macro for config crate

### crates/[prelude](./crates/prelude/README.md)

crate lib to implement shared parts that will be used by all projects crates for code2vault