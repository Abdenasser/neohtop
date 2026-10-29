# Desktop analytics relay

Cloudflare Worker using the free plan only. No KV, D1, paid services, request logs or event storage. GA4's Measurement Protocol API secret stays in a Worker secret. The desktop sends only schema-checked, allowlisted product events; native process/system data never enters the telemetry module.

## Setup

1. In GA4, use a dedicated **Web data stream** for desktop Measurement Protocol events, or explicitly choose the existing project web stream. Create its Measurement Protocol API secret in Analytics Admin. Do not commit or put it in a desktop environment variable.
2. Authorize Wrangler for the intended Cloudflare account, with account/user read and Workers deployment permissions. Stay on Workers Free (100,000 requests/day, 10 ms CPU/request as verified October 29, 2026). No paid upgrade is needed. The free request limit is shared across Workers in that account.
3. From the repository root: `npx wrangler deploy --config telemetry/wrangler.jsonc`. No routes or custom domain are necessary. Confirm `GET /health` works. This deployment returns 503 for events until configured.
4. Set `GA_MEASUREMENT_ID` as a Worker variable. Set `GA_API_SECRET` as an encrypted Worker secret through Cloudflare's dashboard or `wrangler secret put GA_API_SECRET --config telemetry/wrangler.jsonc`. Enter the secret at the prompt; never pass it as a visible command argument. Redeploy preserving the secret if variables change.
5. Set repository Actions variable `NEOHTOP_ANALYTICS_ENDPOINT` to the resulting `https://neohtop-analytics.<account-subdomain>.workers.dev/v1/events`. It is public configuration. All five package builds use it. Local builds omit it by default and send nothing. Release preparation must verify the variable and health check before tagging.
6. Validate a synthetic basic event against GA's **debug** Measurement Protocol endpoint before enabling delivery. Do not send private process samples or fake production sessions. A 204 from the regular GA endpoint does not confirm a valid event. Check real launch delivery in GA Realtime after the configured build is launched.

## Schema and limits

`events.json` is the shared native/server event allowlist. Basic mode accepts exactly schema, mode, event (`desktop_open`) and app version. Optional usage adds random installation ID, one launch-session ID, OS family and a fixed choice where relevant. Unknown keys/events/values are rejected, as are oversized bodies and browser-origin requests. GA receives a new ephemeral client ID for every basic event, so **basic counts cannot measure unique users or installations**. No client IP or headers are forwarded. Ad personalization and ad-user-data consent are denied.

There is no embedded secret in the public app; therefore this public endpoint cannot authenticate a genuine installation. Origin filtering is only browser abuse mitigation. For traffic spikes or invalid data, disable the Worker or rotate the server secret; do not enable paid billing automatically. Free-plan exhaustion drops analytics without affecting local monitoring. Logs/observability remain off; Cloudflare and Google still process transport data under their service policies.

Tests: `npm run test:release` (Node 22+) and `cargo test --manifest-path src-tauri/Cargo.toml`. The first-launch disclosure and Settings Privacy pane must remain consistent with `docs/privacy.md`.
