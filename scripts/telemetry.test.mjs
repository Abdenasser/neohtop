import test from "node:test";
import assert from "node:assert/strict";
import worker, { validateEvent, gaPayload } from "../telemetry/worker.js";
const basic = {
  schema: 1,
  mode: "basic",
  event: "desktop_open",
  version: "1.3.0",
};
const usage = {
  ...basic,
  mode: "usage",
  client_id: "c5ec001c-a4bb-444e-9dce-9027bb550808",
  session_id: 1793000000,
  platform: "macos",
};
const request = (body, headers = {}) =>
  new Request("https://example.workers.dev/v1/events", {
    method: "POST",
    headers: { "content-type": "application/json", ...headers },
    body: typeof body === "string" ? body : JSON.stringify(body),
  });
test("basic launch schema rejects identifiers, readings and extra fields", () => {
  assert.deepEqual(validateEvent(basic), basic);
  for (const key of [
    "client_id",
    "platform",
    "value",
    "session_id",
    "processes",
    "ip_address",
    "search",
    "path",
  ])
    assert.equal(validateEvent({ ...basic, [key]: "private" }), null, key);
  for (const version of [
    null,
    13,
    ["1.3.0"],
    {},
    "latest",
    "1.3.0/private",
    "1.3.0-" + "a".repeat(40),
  ])
    assert.equal(validateEvent({ ...basic, version }), null);
  assert.equal(validateEvent({ ...basic, event: "inspector_open" }), null);
});
test("usage schema accepts only explicit event choices and installation IDs", () => {
  assert.deepEqual(validateEvent(usage), usage);
  assert.ok(validateEvent({ ...usage, event: "resource_view", value: "gpu" }));
  assert.ok(
    validateEvent({ ...usage, event: "theme_changed", value: "catppuccin" }),
  );
  for (const patch of [
    { event: "process_list" },
    { event: "resource_view", value: "private" },
    { value: "extra" },
    { client_id: "hostname" },
    { client_id: [usage.client_id] },
    { session_id: 0 },
    { session_id: Infinity },
    { platform: "my-device-name" },
    { event: "port_search", value: "3000" },
    { event: "theme_changed", value: "not-a-theme" },
  ])
    assert.equal(
      validateEvent({ ...usage, ...patch }),
      null,
      JSON.stringify(patch),
    );
});
test("basic launches use distinct ephemeral IDs and contain no OS or network data", () => {
  const a = gaPayload(basic),
    b = gaPayload(basic);
  assert.notEqual(a.client_id, b.client_id);
  assert.match(a.client_id, /^\d+\.\d+$/);
  assert.equal(a.events[0].name, "desktop_open");
  assert.equal(a.events[0].params.collection_mode, "basic");
  assert.equal(a.events[0].params.platform, undefined);
  assert.equal(a.ip_override, undefined);
  assert.equal(a.user_id, undefined);
  assert.deepEqual(a.consent, {
    ad_user_data: "DENIED",
    ad_personalization: "DENIED",
  });
});
test("usage retains the resettable ID and launch session, never arbitrary payload fields", () => {
  const payload = gaPayload({
    ...usage,
    event: "resource_view",
    value: "memory",
  });
  assert.equal(
    payload.client_id,
    `${Number.parseInt("c5ec001c", 16)}.${Number.parseInt("bb550808", 16)}`,
  );
  assert.match(payload.client_id, /^\d+\.\d+$/);
  assert.equal(payload.events[0].params.session_id, usage.session_id);
  assert.equal(payload.events[0].params.choice, "memory");
  assert.equal(payload.events[0].params.os_family, "macos");
});
test("relay refuses browser origins, oversized streams, invalid JSON and unsupported fields", async () => {
  assert.equal(
    (await worker.fetch(request(basic, { origin: "https://evil.example" }), {}))
      .status,
    403,
  );
  assert.equal((await worker.fetch(request("x".repeat(2049)), {})).status, 413);
  assert.equal((await worker.fetch(request("{"), {})).status, 400);
  assert.equal(
    (await worker.fetch(request({ ...basic, pid: 10 }), {})).status,
    400,
  );
  assert.equal((await worker.fetch(request(basic), {})).status, 503);
});
test("relay forwards only the allowlisted GA event, keeping secret and client headers server-side", async () => {
  const original = globalThis.fetch;
  let sent;
  globalThis.fetch = async (url, init) => {
    sent = { url: String(url), init };
    return new Response(null, { status: 204 });
  };
  try {
    const response = await worker.fetch(
      request(basic, {
        "cf-connecting-ip": "192.0.2.123",
        "user-agent": "private-machine",
      }),
      {
        GA_MEASUREMENT_ID: "G-TEST123",
        GA_API_SECRET: "synthetic-test-secret",
      },
    );
    assert.equal(response.status, 204);
    assert.match(
      sent.url,
      /^https:\/\/www.google-analytics.com\/mp\/collect\?/,
    );
    assert.ok(sent.url.includes("api_secret=synthetic-test-secret"));
    assert.equal(sent.init.redirect, "manual");
    assert.deepEqual(sent.init.headers, { "content-type": "application/json" });
    assert.ok(!sent.init.body.includes("synthetic-test-secret"));
    assert.ok(!sent.init.body.includes("192.0.2.123"));
    assert.ok(!sent.init.body.includes("private-machine"));
    assert.equal(await response.text(), "");
  } finally {
    globalThis.fetch = original;
  }
});
test("relay health is separate from event delivery and collection failures stay generic", async () => {
  assert.equal(
    (await worker.fetch(new Request("https://example.workers.dev/health"), {}))
      .status,
    200,
  );
  assert.equal(
    (await worker.fetch(new Request("https://example.workers.dev/other"), {}))
      .status,
    404,
  );
  const original = globalThis.fetch;
  globalThis.fetch = async () => {
    throw new Error("synthetic-secret-must-not-leak");
  };
  try {
    const result = await worker.fetch(request(basic), {
      GA_MEASUREMENT_ID: "G-TEST",
      GA_API_SECRET: "synthetic",
    });
    assert.equal(result.status, 502);
    assert.ok(!(await result.text()).includes("secret"));
  } finally {
    globalThis.fetch = original;
  }
});

test("validation mode checks the payload with Google without recording a production event", async () => {
  const original = globalThis.fetch;
  let sent;
  globalThis.fetch = async (url, init) => {
    sent = { url: String(url), payload: JSON.parse(init.body) };
    return Response.json({ validationMessages: [] });
  };
  try {
    const env = {
      GA_MEASUREMENT_ID: "G-TEST",
      GA_API_SECRET: "synthetic",
      GA_VALIDATION_ONLY: "true",
    };
    const result = await worker.fetch(request(basic), env);
    assert.equal(result.status, 200);
    assert.deepEqual(await result.json(), { validationMessages: [] });
    assert.ok(
      sent.url.startsWith("https://www.google-analytics.com/debug/mp/collect?"),
    );
    assert.equal(sent.payload.validation_behavior, "ENFORCE_RECOMMENDATIONS");
    const health = await worker.fetch(
      new Request("https://example.workers.dev/health"),
      env,
    );
    assert.deepEqual(await health.json(), {
      service: "NeoHtop analytics relay",
      configured: true,
      validation_only: true,
    });
  } finally {
    globalThis.fetch = original;
  }
});
