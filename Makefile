build:
	docker build . -t cloudflare-dyndns

run: build
	docker run \
	--env CLOUDFLARE_API_TOKEN=XXX \
	--env DOMAIN=acme.com \
	--env RECORD=www \
	--env DEBUG=false \
	--env CHECK_INTERVAL_SEC=900 \
	cloudflare-dyndns