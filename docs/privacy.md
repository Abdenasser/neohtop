# NeoHtop privacy notice

Updated October 29, 2026. Covers NeoHtop Desktop v1.3.0 and this project website. NeoHtop Pro is postponed and is not available.

## Local monitoring

Processes, commands, search text, file paths, environment values, socket/port numbers and CPU, memory, storage, network and GPU readings stay on your machine. NeoHtop does not upload these readings. Monitoring works offline and requires no account.

## Basic desktop launch counter

Official builds configured with the project analytics relay send one `desktop_open` event when the app starts. It contains only the event name, app version, schema version and the collection mode. It is sent even when optional feature usage is off. It has no persistent client identifier, machine identifier or OS field, and does not count distinct people or installations. Reopening a hidden window does not send another launch event.

The request goes over HTTPS to the project's Cloudflare Worker, which forwards a limited event to Google Analytics 4. Cloudflare receives the connecting IP address to handle the request. The Worker does not forward the client IP address or request headers to Google, write request logs, or store events. It generates a new random GA client ID for each basic launch event, which cannot connect different launches. Cloudflare and Google also process data according to their own service policies; this notice does not claim that network requests are anonymous to the service providers.

## Optional feature usage

**Settings → Privacy → Share feature usage** is off by default. When enabled, it adds your OS family (macOS, Windows or Linux), a random installation identifier and a launch-session identifier. The installation identifier is saved locally and deleted when sharing is turned off; enabling it again creates a new one. Turning sharing off stops future optional events; it does not erase data already received by Google.

Only these events are allowed: app launch, opening a named resource view, choosing one of the built-in themes, switching process list/tree view, opening the companion, inspector or Settings, and using port search. Port search sends only that the feature was used, never its query or port. Rapid repeats are limited. The native app and relay both reject arbitrary event names and values. Advertising personalization and ad-user-data consent are denied in forwarded events.

Delivery happens in the background with a short timeout. There is no stored event queue or retry. Blocked analytics and unavailable networks do not prevent monitoring. A launch event records an attempted use of the app, not a guaranteed successful session.

## Website analytics

The production website uses Google Analytics 4 with measurement ID `G-88HFXRBNLS` for page usage. Google's browser tag can set analytics cookies and receives browser/network information according to Google's policies. It is separate from desktop sharing preferences. Analytics does not load on localhost. Blocking it does not affect reading the website or downloading NeoHtop.

## Self-built copies

Desktop analytics requires a build-time `NEOHTOP_ANALYTICS_ENDPOINT` pointing to the HTTPS relay. Builds without it send no desktop events and show this in Settings. The Google API secret is held only in the Worker, never in app binaries or source control. See the [relay setup guide](https://github.com/Abdenasser/neohtop/blob/main/telemetry/README.md).

## Questions

For questions, open a [project issue](https://github.com/Abdenasser/neohtop/issues), without posting private process details or credentials. Review the [Cloudflare privacy policy](https://www.cloudflare.com/privacypolicy/) and [Google privacy policy](https://policies.google.com/privacy).
