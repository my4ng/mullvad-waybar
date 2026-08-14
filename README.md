<h1 align="center">
mullvad-waybar
</h1>

**NOTE: Mullvad VPN AB holds the rights to the “Mullvad VPN” brand and logos. This project is not affiliated or sponsored by Mullvad VPN AB.**

Simple yet configurable [waybar](https://github.com/alexays/waybar) custom module for Mullvad VPN.

## Dependency

- `mullvad` binary
- [nerd font](https://www.nerdfonts.com/#home) (optional)

## Installation

```
git clone https://github.com/my4ng/mullvad-waybar.git
cd mullvad-waybar
cargo build --release
```

Copy the built binary to a `$PATH` location, for example:

```
sudo cp target/release/mullvad-waybar /usr/local/bin
```

## Usage

```
Usage: mullvad-waybar [OPTIONS]

Options:
  -b, --binary <BINARY>  Path to the Mullvad VPN binary
  -h, --help             Print help
  -V, --version          Print version
```

By default `mullvad-waybar` will use the `mullvad` binary found in `$PATH`. If a different one is required, use the `--binary` option.

## Waybar example configuration & styling

Configuration (requires nerd font support for icon):

```
"custom/mullvad-waybar": {
    "format": "{icon}{text}",
    "format-icons": {
        "offline": "󰣯",
        "connected": "󰌾",
        "connected-lockdown": "󱎚",
        "disconnected": "󰌿",
        "disconnected-lockdown": "󱚰"
    },
    "tooltip": true,
    "exec": "mullvad-waybar",
    "return-type": "json"
}
```

CSS classes `offline`, `connected`, `wireguard`, `lockdown` are supported (default being disconnected). For example:

```
#custom-mullvad-waybar {
	color: #e6e6e6;
}

#custom-mullvad-waybar.offline {
	color: #f44336;
}
```

## License

The project is licensed under the BSD-2-Clause Plus Patent license, see [LICENSE](LICENSE) for more details.

_Note: This license is designed to provide: a) a simple permissive license; b) that is compatible with the GNU General Public License (GPL), version 2; and c) which also has an express patent grant included._
