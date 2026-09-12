# Cloudflare DynDNS

A small Rust binary (packaged as a Docker image) that updates a Cloudflare DNS A record to the public IP of the host it runs on.

Useful when your system has a dynamic IP (typical home / ISP connection).

## How to use

### Docker-compose

Copy the `docker-compose.yml` file and set the environment variables:

- **CLOUDFLARE_API_TOKEN**: Your token from Cloudflare. You can create one [here](https://dash.cloudflare.com/profile/api-tokens).
- **DOMAIN**: The domain name (zone), for example: `acme.com`.
- **RECORD**: The DNS record label without the domain, for example `www`. For the root record, use an empty string.
- **CHECK_INTERVAL_SEC**: Seconds between IP checks. Defaults to `900` (15 min).
- **DEBUG**: Set to `true` for extra logging.

### Docker

```
docker run \
--env CLOUDFLARE_API_TOKEN=XXX \
--env DOMAIN=acme.com \
--env RECORD=www \
--env DEBUG=false \
--env CHECK_INTERVAL_SEC=900 \
antoinejaussoin/cloudflare-dyndns:latest
```

Image tags: `latest` and the semver from `Cargo.toml`.
