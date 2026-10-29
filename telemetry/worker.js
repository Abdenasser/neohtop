import events from "./events.json" with { type: "json" };
const headers = {
  "cache-control": "no-store",
  "content-type": "application/json",
};
const reply = (status, message) =>
  new Response(JSON.stringify({ message }), { status, headers });
export function validateEvent(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  const allowed = [
    "schema",
    "mode",
    "event",
    "version",
    "value",
    "client_id",
    "session_id",
    "platform",
  ];
  if (Object.keys(value).some((key) => !allowed.includes(key))) return null;
  if (
    value.schema !== 1 ||
    typeof value.version !== "string" ||
    !/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[A-Za-z0-9.-]+)?$/.test(
      value.version || "",
    ) ||
    value.version.length > 40
  )
    return null;
  if (!Object.hasOwn(events, value.event)) return null;
  if (value.mode === "basic") {
    if (
      value.event !== "desktop_open" ||
      Object.keys(value).some(
        (key) => !["schema", "mode", "event", "version"].includes(key),
      )
    )
      return null;
    return {
      schema: 1,
      mode: "basic",
      event: "desktop_open",
      version: value.version,
    };
  }
  if (
    value.mode !== "usage" ||
    typeof value.client_id !== "string" ||
    !/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(
      value.client_id || "",
    )
  )
    return null;
  if (
    !Number.isSafeInteger(value.session_id) ||
    value.session_id <= 0 ||
    !["macos", "linux", "windows"].includes(value.platform)
  )
    return null;
  if (
    events[value.event].length
      ? !events[value.event].includes(value.value)
      : value.value !== undefined
  )
    return null;
  return { ...value };
}
export function gaPayload(event, random = () => crypto.randomUUID()) {
  const params = {
    app_version: event.version,
    collection_mode: event.mode,
    engagement_time_msec: 1,
  };
  if (event.mode === "usage") {
    params.session_id = event.session_id;
    params.platform = event.platform;
    if (event.value !== undefined) params.choice = event.value;
  } else params.session_id = Math.floor(Date.now() / 1000);
  return {
    client_id: event.mode === "basic" ? random() : event.client_id,
    non_personalized_ads: true,
    consent: { ad_user_data: "DENIED", ad_personalization: "DENIED" },
    events: [{ name: event.event, params }],
  };
}
export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    if (request.method === "GET" && url.pathname === "/health")
      return reply(200, "NeoHtop analytics relay");
    if (request.method !== "POST" || url.pathname !== "/v1/events")
      return reply(404, "Not found");
    if (request.headers.has("origin"))
      return reply(403, "Desktop requests only");
    if (!request.headers.get("content-type")?.startsWith("application/json"))
      return reply(415, "JSON required");
    if (Number(request.headers.get("content-length") || 0) > 2048)
      return reply(413, "Too large");
    const reader = request.body?.getReader();
    if (!reader) return reply(400, "Empty request");
    const chunks = [];
    let length = 0;
    try {
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        length += value.byteLength;
        if (length > 2048) {
          await reader.cancel();
          return reply(413, "Too large");
        }
        chunks.push(value);
      }
    } catch {
      return reply(400, "Unreadable request");
    }
    const bytes = new Uint8Array(length);
    let offset = 0;
    for (const chunk of chunks) {
      bytes.set(chunk, offset);
      offset += chunk.byteLength;
    }
    const body = new TextDecoder().decode(bytes);
    let parsed;
    try {
      parsed = JSON.parse(body);
    } catch {
      return reply(400, "Invalid JSON");
    }
    const event = validateEvent(parsed);
    if (!event) return reply(400, "Unsupported event");
    if (
      !/^G-[A-Z0-9]+$/.test(env.GA_MEASUREMENT_ID || "") ||
      !env.GA_API_SECRET
    )
      return reply(503, "Analytics not configured");
    const target = new URL("https://www.google-analytics.com/mp/collect");
    target.searchParams.set("measurement_id", env.GA_MEASUREMENT_ID);
    target.searchParams.set("api_secret", env.GA_API_SECRET);
    try {
      const response = await fetch(target, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(gaPayload(event)),
        redirect: "error",
        signal: AbortSignal.timeout(4000),
      });
      return response.ok
        ? new Response(null, {
            status: 204,
            headers: { "cache-control": "no-store" },
          })
        : reply(502, "Collection unavailable");
    } catch {
      return reply(502, "Collection unavailable");
    }
  },
};
