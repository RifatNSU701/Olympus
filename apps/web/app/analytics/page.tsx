'use client';

import { useEffect, useMemo, useState } from "react";
import styles from "./analytics.module.css";

type RevenuePoint = { month: string; revenue: string | number };
type TopProduct = { name: string; revenue: string | number; units_sold: number };
type CategoryMix = { name: string; revenue: string | number };
type Analytics = {
  revenue: string | number;
  orders: number;
  average_order_value: string | number;
  revenue_trend: RevenuePoint[];
  top_products: TopProduct[];
  category_mix: CategoryMix[];
};

const API = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8080";

function amount(value: string | number) {
  return Number(value ?? 0);
}

function money(value: string | number) {
  const numeric = amount(value);
  return `৳${numeric.toLocaleString("en-BD", { maximumFractionDigits: 0 })}`;
}

function shortMoney(value: string | number) {
  const numeric = amount(value);
  if (numeric >= 1_000_000) return `৳${(numeric / 1_000_000).toFixed(2)}M`;
  if (numeric >= 1_000) return `৳${(numeric / 1_000).toFixed(1)}K`;
  return money(numeric);
}

function monthLabel(value: string) {
  const date = new Date(`${value}-01T00:00:00`);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString("en-US", { month: "short" });
}

export default function AnalyticsPage() {
  const [data, setData] = useState<Analytics | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    const token = window.localStorage.getItem("olympus_token");
    if (!token) {
      setError("Sign in as a seller to view live analytics.");
      setLoading(false);
      return;
    }

    const controller = new AbortController();
    fetch(`${API}/api/v1/seller/analytics`, {
      headers: { Authorization: `Bearer ${token}` },
      signal: controller.signal,
    })
      .then(async (response) => {
        if (!response.ok) {
          if (response.status === 401 || response.status === 403) throw new Error("Your seller session is not authorized for analytics.");
          throw new Error("Unable to load seller analytics right now.");
        }
        return response.json() as Promise<Analytics>;
      })
      .then(setData)
      .catch((requestError: unknown) => {
        if (requestError instanceof DOMException && requestError.name === "AbortError") return;
        setError(requestError instanceof Error ? requestError.message : "Unable to load seller analytics right now.");
      })
      .finally(() => setLoading(false));

    return () => controller.abort();
  }, []);

  const maxRevenue = useMemo(
    () => Math.max(...(data?.revenue_trend ?? []).map((item) => amount(item.revenue)), 1),
    [data],
  );

  const categoryTotal = useMemo(
    () => (data?.category_mix ?? []).reduce((sum, item) => sum + amount(item.revenue), 0),
    [data],
  );

  if (loading) {
    return <main className={styles.page}><div className={styles.state}><span className={styles.eyebrow}>Seller intelligence</span><h1>Loading analytics…</h1><p>Fetching your latest marketplace performance.</p></div></main>;
  }

  if (error || !data) {
    return <main className={styles.page}><div className={styles.state}><span className={styles.eyebrow}>Seller intelligence</span><h1>Analytics unavailable</h1><p>{error || "No analytics data is available yet."}</p></div></main>;
  }

  const metrics = [
    { label: "Gross merchandise value", value: shortMoney(data.revenue) },
    { label: "Orders", value: data.orders.toLocaleString("en-BD") },
    { label: "Average order value", value: money(data.average_order_value) },
    { label: "Top product revenue", value: shortMoney(data.top_products[0]?.revenue ?? 0) },
  ];

  return (
    <main className={styles.page}>
      <header className={styles.header}>
        <div>
          <span className={styles.eyebrow}>Seller intelligence</span>
          <h1>Analytics overview</h1>
          <p>Understand marketplace performance, demand and product momentum from live order data.</p>
        </div>
        <span className={styles.period}>Last 6 months <span>⌄</span></span>
      </header>

      <section className={styles.metricGrid} aria-label="Key performance indicators">
        {metrics.map((metric) => (
          <article className={styles.metric} key={metric.label}>
            <span>{metric.label}</span>
            <strong>{metric.value}</strong>
            <small className={styles.up}>Live data <i>from seller orders</i></small>
          </article>
        ))}
      </section>

      <section className={styles.grid}>
        <article className={`${styles.card} ${styles.revenue}`}>
          <div className={styles.cardHead}>
            <div><span className={styles.eyebrow}>Performance</span><h2>Revenue trend</h2></div>
            <span className={styles.badge}>{shortMoney(data.revenue)} total</span>
          </div>
          <div className={styles.chart} aria-label="Revenue trend for the last six months">
            <div className={styles.gridLines}><span/><span/><span/><span/></div>
            <div className={styles.bars}>
              {data.revenue_trend.map((item) => (
                <div className={styles.barWrap} key={item.month} title={`${monthLabel(item.month)}: ${money(item.revenue)}`}>
                  <div className={styles.bar} style={{ height: `${Math.max((amount(item.revenue) / maxRevenue) * 100, 4)}%` }} />
                  <span>{monthLabel(item.month)}</span>
                </div>
              ))}
            </div>
          </div>
        </article>

        <article className={styles.card}>
          <div className={styles.cardHead}><div><span className={styles.eyebrow}>Mix</span><h2>Sales by category</h2></div></div>
          <div className={styles.categoryList}>
            {data.category_mix.length === 0 ? <p>No categorized sales yet.</p> : data.category_mix.map((item) => {
              const percentage = categoryTotal ? Math.round((amount(item.revenue) / categoryTotal) * 100) : 0;
              return <div className={styles.category} key={item.name}><div><span>{item.name}</span><b>{percentage}%</b></div><div className={styles.track}><span style={{ width: `${percentage}%` }}/></div></div>;
            })}
          </div>
        </article>
      </section>

      <section className={styles.card}>
        <div className={styles.cardHead}><div><span className={styles.eyebrow}>Product intelligence</span><h2>Top products</h2></div><span className={styles.badge}>Live</span></div>
        <div className={styles.table} id="products">
          <div className={`${styles.tableRow} ${styles.tableHeader}`}><span>Product</span><span>Revenue</span><span>Units sold</span></div>
          {data.top_products.length === 0 ? <p>No completed seller sales yet.</p> : data.top_products.map((product, index) => (
            <div className={styles.tableRow} key={product.name}><span><em>{String(index + 1).padStart(2, "0")}</em>{product.name}</span><strong>{money(product.revenue)}</strong><span>{product.units_sold.toLocaleString("en-BD")}</span></div>
          ))}
        </div>
      </section>
    </main>
  );
}
