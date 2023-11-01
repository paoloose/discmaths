import { useEffect, useState } from "react";
import { vennfetch } from "./fetching";

enum Status {
  CONNECTING,
  CONNECTED,
  DOWN,
}

const statusMap: Record<Status, { class: string; text: string }> = {
  [Status.CONNECTING]: {
    class: "connecting",
    text: "Connecting...",
  },
  [Status.CONNECTED]: {
    class: "connected",
    text: "Connected",
  },
  [Status.DOWN]: {
    class: "down",
    text: "Down",
  },
}

function VennbaseStatus() {
  const [status, setStatus] = useState<Status>(Status.CONNECTING);

  useEffect(() => {
    vennfetch("/health")
      .then(res => res.status === 200 ? Status.CONNECTED : Status.DOWN)
      .catch(() => Status.DOWN)
      .then((s) => setTimeout(() => setStatus(s), 1069))
  });

  return (
    <span className="vennbase-status">
      <span className="status-badge">Status</span>
      <span className={`status-text ${statusMap[status].class}`}>{statusMap[status].text}</span>
    </span>
  );
}

export default VennbaseStatus;
