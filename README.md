# What this is
`Hello, world!` from WearOS example

# run
Connect WearOS by ADB

Install `cargo dinghy` and
```bash
 adb devices
List of devices attached
192.168.11.32:5555      device

cargo dinghy -d 192.168.11.32:5555 run
cargo dinghy -d 192.168.11.32:5555 run --release
```
