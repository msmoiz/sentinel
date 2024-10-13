/** An enumeration of available metric types. */
export type MetricType = "cpu_usage" | "mem_usage" | "disk_usage";

/** A metric. */
export interface Metric {
  /** The time associated with the metric. */
  time: Date;
  /** The value of the metric. */
  value: number;
}

/** Gets datapoints for the provided metric. */
export async function getMetrics(name: MetricType): Promise<Metric[]> {
  const response = await fetch("/get-metrics", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ name }),
  });

  const body = await response.json();

  const metrics: Metric[] = [];

  for (const metric of body["metrics"]) {
    const time = new Date(metric[0] * 1000);
    const value = metric[1];
    metrics.push({ time, value });
  }

  return metrics;
}
