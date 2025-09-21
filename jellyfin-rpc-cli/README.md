# Jellyfin-RPC (ImgBB Fork)

## How to Use

You can get your API key [here](https://api.imgbb.com/).

The configuration has changed just a bit. You can check the difference in [example.json](https://github.com/NormalFall/jellyfin-rpc/blob/main/example.json).
The expiration time is fully optional and if not used it will default to `432000` seconds.
```json
{
    Other Configs... 
    "imgbb": {
        "api_token": "asdjdjdg394209fdjs093",
        "expiration": 432000
    },
    "images": {
        "enable_images": true,
        "imgbb_images": true
    }
}
```

> **_Notice:_** Imgur support has been **FULLY** removed

## Extra Features

### NixOS Support
You can insert this code into your `home manager` config to run it as a service.
```nix
services.jellyfin-rpc = {
    enable = true;
    jellyfinSecretPath = "${config.xdg.configHome}/jellyfin-rpc/jellyfin.secret"; # Recommended if you're not using secret manager
    imgbbSecretPath = "${config.xdg.configHome}/jellyfin-rpc/imgbb.secret"; # Recommended if you're not using secret manager
    config = {
        jellyfin = {
            url = "https://example.com";
            api_key = "sadasodsapasdskd";
            username = "your_username_here";
            self_signed_cert = false;
            show_simple = false;
        };

        discord = {
            application_id = "1053747938519679018";
            show_paused = true;
        };

        imgbb = {
            api_token = "asdjdjdg394209fdjs093";
            expiration = 432000;
        };

        images = {
            enable_images = true;
            imgbb_images = true;
        };
    };
};
```
### Separate Secret Files
If for some reason you need to store API keys in a separate file from the config:
```bash
jellyfin-rpc -a /path/to/imgbb_api_key.secret
jellyfin-rpc -j /path/to/jellyfin_api_key.secret
```

### TODO

- Add Nix support for more platforms
- Make a better blacklist system