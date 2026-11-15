FROM alpine:3.20
RUN apk add --no-cache nftables iproute2
COPY rules.nft /etc/nftables/rules.nft
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
