build:
	docker build . -t cloudflare-dyndns

run: build
	docker run cloudflare-dyndns