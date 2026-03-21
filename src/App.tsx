import { useEffect, useState } from "react";
import {
  getMonitors,
  toGetMonitorsError,
  type GetMonitorsError,
  type GetMonitorsResponse,
  type MonitorIdentity,
} from "./api/monitorApi";

function App() {
  const [data, setData] = useState<GetMonitorsResponse | null>(null);
  const [error, setError] = useState<GetMonitorsError | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    let cancelled = false;

    void (async () => {
      setLoading(true);
      setData(null);
      setError(null);

      try {
        const result = await getMonitors();

        if (!cancelled) {
          setData(result);
        }
      } catch (e: unknown) {
        if (!cancelled) {
          setError(toGetMonitorsError(e));
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    })();

    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <main style={{ padding: 24, fontFamily: "sans-serif" }}>
      <h1>Monitor List</h1>

      {loading && <p>loading...</p>}

      {error?.type === "MonitorsNotFound" && (
        <p style={{ color: "red" }}>モニタが見つかりませんでした。</p>
      )}

      {error?.type === "Unavailable" && (
        <p style={{ color: "red" }}>モニタ取得に失敗しました。</p>
      )}

      {error?.type === "TransportError" && (
        <p style={{ color: "red" }}>
          接続エラー: {error.message}
        </p>
      )}

      {data && (
        <ul>
          {data.monitors.map((monitor, index) => (
            <li key={monitor.monitor_id}>
              {index + 1}: {toMonitorLabel(monitor)}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}

function toMonitorLabel(monitor: MonitorIdentity): string {
  const friendlyName = normalizeNullableString(monitor.friendly_name);
  if (friendlyName) {
    return friendlyName;
  }

  if (monitor.edid) {
    const vendor = normalizeNullableString(monitor.edid.vendor);
    const productId = monitor.edid.product_id
      .toString(16)
      .toUpperCase()
      .padStart(4, "0");

    if (vendor) {
      return `${vendor}-${productId}`;
    }

    return `Unknown-${productId}`;
  }

  return "不明なモニタ";
}

function normalizeNullableString(value: string | null | undefined): string | null {
  if (typeof value !== "string") {
    return null;
  }

  const trimmed = value.trim();
  return trimmed.length > 0 ? trimmed : null;
}

export default App;