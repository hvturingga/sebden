# Infrastructure

## Database

```shell
export DATABASE_URL=postgres://username:password@host/juson
```

```shell
sqlx database create
```

```shell
sqlx migrate add create_initial_tables
```

```shell
sqlx migrate run
```

```shell
sqlx migrate revert
```


```shell
cargo sqlx prepare
```