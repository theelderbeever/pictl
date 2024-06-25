# pictl

A cli for using the pihole [api](https://github.com/pi-hole/web/blob/master/api.php) and eventually the [db api](https://github.com/pi-hole/web/blob/master/api_db.php). Many endpoints aren't implemented yet.

## Installation

Right now this requires Rust's `cargo` to install however, I plan on building binaries later.

```bash
cargo install --git ssh://git@github.com/theelderbeever/pictl.git
```

## Configure

On the device running pihole run

```bash
grep 'WEBPASSWORD=' /etc/pihole/setupVars.conf | awk -F'=' '{print $2}'
```

On your remote device create the file `~/.pirc` and fill it in with the following replaceing `<PASSWORD_HASH>` with the value you got from above.

```toml
admin_url = "http://pi.hole/admin/api.php"
pwhash = "<PASSWORD_HASH>"
```

## Usage

Easiest starting point is to just check the help (`pictl -h`). Subcommands also have help (`pictl list -h`).

I primarily use this to enable/disable blocking temporarily

```bash
❯ pictl disable 900
{"status":"disabled"}

❯ pictl enable
{"status":"enabled"}
```

You can also view your lists

```bash
❯ pictl list white show | jq .data
{
  "data": [
    {
      "id": 3,
      "type": 0,
      "domain": "api2.branch.io",
      "enabled": 1,
      "date_added": 1658845072,
      "date_modified": 1658845107,
      "comment": "Needed to redirect from Chrome to Reddit app on mobile 👎",
      "groups": []
    }
  ]
}
```
