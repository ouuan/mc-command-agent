# MC Command Agent

Give users permission to run specific Minecraft commands, so that you don't need to be pinged to run commands like tp.

It has weak security mechanisms (e.g. HTTP basic auth with plaintext password storage), so don't use it if security is important, and please use random passwords.

It uses a simple config file and no database, so it's not suitable for big servers with many players.

## Run

```sh
cargo build --release
target/release/mc-command-agent
```

## Config

Example `config.yaml`:

```yaml
server_host: 0.0.0.0:12345
rcon_host: 127.0.0.1:25575
rcon_password: password
users:
  - username: alice
    password: password
    groups:
      - alice-group
  - username: bob
    password: password
    groups:
      - bob-group
groups:
  - name: alice-group
    commands:
      - tp alice bob
      - kill alice
  - name: bob-group
    commands:
      - tp bob alice
      - kill bob
```
