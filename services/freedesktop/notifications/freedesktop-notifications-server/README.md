# freedesktop-notifications-server
A rust library that implements the D-Bus interface for the Freedesktop Notifications Server.

# Features
- Implements the D-Bus interface for the Freedesktop Notifications Server.
- Can use simple trait invoke actions when any of these occurs: send, replace, and close notifications.
- Supports asynchronous operations using `async_trait`.

# Example Usage
1. First kill any existing notification servers:
To get the PID and application name
```bash
busctl --user status org.freedesktop.Notifications
ps -p <PID> -o comm=
```

``` bash
cargo run --example recv_print
```
