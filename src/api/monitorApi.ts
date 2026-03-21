import { invoke } from "@tauri-apps/api/core";

export type EdidSummary = {
  identifier: string;
  vendor: string;
  product_id: number;
  serial: number;
  week: number;
  year: number;
};

export type MonitorIdentity = {
  monitor_id: string;
  friendly_name: string | null;
  edid: EdidSummary | null;
};

export type GetMonitorsResponse = {
  monitors: MonitorIdentity[];
};

export type GetMonitorsError =
  | { type: "MonitorsNotFound" }
  | { type: "Unavailable" }
  | { type: "TransportError"; message: string };

export async function getMonitors(): Promise<GetMonitorsResponse> {
  try {
    return await invoke<GetMonitorsResponse>("get_monitors_command");
  } catch (e: unknown) {
    throw toGetMonitorsError(e);
  }
}

export function toGetMonitorsError(e: unknown): GetMonitorsError {
  if (isTypedError(e, "MonitorsNotFound")) {
    return e;
  }

  if (isTypedError(e, "Unavailable")) {
    return e;
  }

  if (isTransportError(e)) {
    return e;
  }

  return {
    type: "TransportError",
    message: e instanceof Error ? e.message : String(e),
  };
}

export function isGetMonitorsError(value: unknown): value is GetMonitorsError {
  return (
    isTypedError(value, "MonitorsNotFound") ||
    isTypedError(value, "Unavailable") ||
    isTransportError(value)
  );
}

function isTransportError(
  value: unknown,
): value is Extract<GetMonitorsError, { type: "TransportError" }> {
  return (
    typeof value === "object" &&
    value !== null &&
    "type" in value &&
    "message" in value &&
    (value as { type?: unknown }).type === "TransportError" &&
    typeof (value as { message?: unknown }).message === "string"
  );
}

function isTypedError<TType extends GetMonitorsError["type"]>(
  value: unknown,
  type: TType,
): value is Extract<GetMonitorsError, { type: TType }> {
  return (
    typeof value === "object" &&
    value !== null &&
    "type" in value &&
    (value as { type?: unknown }).type === type
  );
}