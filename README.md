# Development board API (Rust)

A dashboard for organizing software development tasks (Kanban flow) implemented in Rust

### Front-end

[Here](https://github.com/goto-eof/dev_board_react) you can find the front-end application.

### Run project

Start DBMS container:

```
docker-compose up
```

Start server:

```
cargo run
```

The server will be reacheable at:

```
http://127.0.0.1:8013
```

### Docker (production)

```
docker-compoer -f docker-compose-production.yml up
```

The server will be reacheable at:

```
http://127.0.0.1:8013
```

### Postman

Import postman collection file from test/postma.json in your postman workspace.

### Technologies

- warp
- sea-orm
- tokio
- postgres

### DB schema

![db schema](db-schema1.png)

### Default user

```
username: admin
password: password
```

### TODO

- ~~be/fe - optimize front-end/back-end in order to understand better if it is a server down issue or the user is not logged in (show a toast for example). Improve json response on the backend side (uniform responses);~~
- fe - optimize login and registration forms of the front-end;
- fe - hide login and register buttons when user is logged in;
- fe - improve form validation
- ~~be - optimize server responses (CORS error when user is not authorized);~~
- ~~be - refactor;~~
- ~~be - implement controllers for permission and role tables (the idea is to have a control panel where it is possible to assign roles and permissions to users);~~
- be - manage better code unwrapping;
- be - improve security;
- be - association of kanban flow to user
- ~~be - fix column/item swapping~~;

### Post scriptum

- this pplication was tested on macOS and Linux
- remember to drop database if application not works (perhaps I made some DDL changes)


