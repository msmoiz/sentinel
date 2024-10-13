import { useEffect, useState } from "react";
import "./App.css";
import { getMetrics, Metric, MetricType } from "./client";
import { Area, AreaChart, CartesianGrid, XAxis, YAxis } from "recharts";

function App() {
  return (
    <>
      <h1>System Monitor</h1>
      <MetricGraph metric="cpu_usage" title="CPU usage" />
      <MetricGraph metric="mem_usage" title="Memory usage" />
      <MetricGraph metric="disk_usage" title="Disk usage" />
    </>
  );
}

interface MetricGraphProps {
  /** The type of metric to graph. */
  metric: MetricType;
  /** The graph title. */
  title: string;
}

/**
 * A time-series graph for a metric.
 */
function MetricGraph(props: MetricGraphProps) {
  const [metrics, setMetrics] = useState<Metric[]>([]);

  // Reload metrics every 5 seconds.
  useEffect(() => {
    const intervalId = setInterval(() => refresh(), 5000);
    return () => clearInterval(intervalId);
  }, []);

  // Refresh metrics.
  async function refresh() {
    const TEN_MIN_SECS = 600;
    const _metrics = await getMetrics(props.metric);
    const slice = _metrics.slice(_metrics.length - TEN_MIN_SECS);
    console.log("metrics len", _metrics.length, slice.length);
    setMetrics(slice);
  }

  return (
    <>
      <h2>{props.title}</h2>
      <AreaChart width={800} height={200} data={metrics}>
        <CartesianGrid strokeDasharray="3 3" stroke="#ccc" />
        <XAxis
          dataKey="time"
          tickFormatter={(value: Date) =>
            `${
              value.getHours() > 12 ? value.getHours() - 12 : value.getHours()
            }:${value.getMinutes()}`
          }
        />
        <YAxis tickFormatter={(value) => `${value.toFixed(0)}%`} />
        <Area
          type="monotone"
          dataKey="value"
          stroke="#8884d8"
          fill="#8884d8"
          isAnimationActive={false}
        />
      </AreaChart>
    </>
  );
}

export default App;
