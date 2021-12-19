# Cloudflare DynDNS

## What is it?

A docker image that updates a specific DNS entry on Cloudflare to the public IP of the system it's running on.

This is useful to update a DNS entry when your system has a dynamic IP (usually when using a regular internet connection).

## How to use

### Docker-compose

Copy paste the `docker-compose.yml` file and update the relevant environement variables:
- **CLOUDFLARE_API_TOKEN**: Your token from Cloudflare. You can get it [here](https://dash.cloudflare.com/profile/api-tokens).
- **DOMAIN**: The domain name, also known as the Zone, for example: `acme.com`.
- **RECORD**: The DNS record, without the domain, for example `www`. For the root, use an empty string. 
- **CHECK_INTERVAL_SEC**: The interval, in seconds, between checking that your IP did not change. Defaults to `900` (15 min).
- **DEBUG**: Set to true to get additional logging in the console.

### Docker

Run:

```
docker run \
--env CLOUDFLARE_API_TOKEN=XXX \
--env DOMAIN=acme.com \
--env RECORD=www \
--env DEBUG=false \
--env CHECK_INTERVAL_SEC=900 \
antoinejaussoin/cloudflare-dyndns:latest
```