# DESIGN — Design

English · [Español](../es/DESIGN.md) · [简体中文](../DESIGN.md)

## Design Goals

- Provide the basic capability and integration method for letting system services run commands as root on their behalf without depending on root tools. The core itself ships no concrete features.
- The command channels impose no privilege requirements on callers (the `heraldctl` command line; the paired front-end App), and execute commands on their behalf from an allowlist.
- The authentication engine takes over KeyMint from the keystore2 backend and provides device key attestation. It is integrated as a plugin rather than built into the core.
- Decoupled from the base image: the core is a shell script plus one `resetprop`, so it needs no changes when the GSI is swapped.

Non-goals: the core does not implement concrete features such as spoofing, battery, or performance. These are provided as plugins by the respective feature patches.

## Architecture

| Module | Responsibility |
|---|---|
| broker (`patch/files/herald-start.sh`) | Runs in the separate SELinux domain `herald`, started after boot_completed. Polls request files and executes commands on their behalf from an allowlist |
| `heraldctl` (`patch/files/heraldctl`) | Unprivileged command-line front end; hands requests to the broker over the command channel |
| Plugin interface | Other patches place files into the plugin directory to add commands, daemons, and self-tests to the broker and heraldctl |
| Authentication engine (`engine/`, HeraldKeymint) | `keymint` (KeyMint daemon, keybox, local RKP) and `inject` (injects into keystore2 via ptrace); forked from Oh My Keymint |

There are two command channels: the command line (`/data/local/tmp/herald`, writable by shell, holds only req/resp) and the paired front-end App (device-random key + uid check). Responses are written to a temporary file first and then renamed, so a reader never sees a partially written file.

Top-level directory responsibilities:

- `patch/` — Source for the core GSI patch (broker, heraldctl, sepolicy, init). Build output: `make dist` → `dist/herald-core/`.
- `engine/` — Source for the authentication engine (a fork; see `engine/README.herald.md`).
- `docs/` — Project documentation.

## Design Constraints

- The plugin interface (the broker's `heraldp_*` and variables; heraldctl's `send` / `heraldctl_cmd_*`) is a contract that other patches depend on. Changes must remain compatible.
- The response working directory must be `/data/local/tmp`, because the shell domain must be enforcing to read responses labeled `shell_data_file`. Placing it in `/data/adb` makes responses unreadable due to an SELinux label mismatch (verified on a real device in V1.2).
- The broker domain is permissive, but the channel directories hold only req/resp, and logs are isolated from the working directory to prevent symlink hijacking of root writes.

## External Interface

The front-end App sends and receives commands over the paired channel (`files/herald_req`, random key + uid check). For the protocol, see the project backend interface documentation. The command-line channel is for uid 0/2000.

## Extensions

For the plugin mechanism, see `docs/plugin-api.md`: broker plugins (`heraldp_cmd_*`, etc.) and heraldctl plugins (`heraldctl_cmd_*`), along with the conventions for payload directory labels and for appending SELinux rules.
