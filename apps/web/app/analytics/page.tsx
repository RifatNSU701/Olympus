import styles from "./analytics.module.css";

const metrics = [
  { label: "Gross merchandise value", value: "৳12.84M", delta: "+18.6%", positive: true },
  { label: "Orders", value: "3,842", delta: "+12.4%", positive: true },
  { label: "Average order value", value: "৳3,341", delta: "+5.8%", positive: true },
  { label: "Conversion rate", value: "4.82%", delta: "-0.7%", positive: false },
];

const months = [
  { month: "Apr", value: 42 },
  { month: "May", value: 48 },
  { month: "Jun", value: 54 },
  { month: "Jul", value: 63 },
  { month: "Aug", value: 71 },
  { month: "Sep", value: 82 },
];

const products = [
  ["Apex Wireless Headphones", "৳2.91M", "742"],
  ["Vertex Mechanical Keyboard", "৳2.34M", "391"],
  ["Atlas Everyday Backpack", "৳1.88M", "614"],
  ["Nova Smart Lamp", "৳1.24M", "288"],
];

export default function AnalyticsPage() {
  return (
    <main className={styles.page}>
      <header className={styles.header}>
        <div>
          <span className={styles.eyebrow}>Seller intelligence</span>
          <h1>Analytics overview</h1>
          <p>Understand marketplace performance, demand and product momentum at a glance.</p>
        </div>
        <button className={styles.period}>Last 6 months <span>⌄</span></button>
      </header>

      <section className={styles.metricGrid} aria-label="Key performance indicators">
        {metrics.map((metric) => (
          <article className={styles.metric} key={metric.label}>
            <span>{metric.label}</span>
            <strong>{metric.value}</strong>
            <small className={metric.positive ? styles.up : styles.down}>{metric.delta} <i>vs previous period</i></small>
          </article>
        ))}
      </section>

      <section className={styles.grid}>
        <article className={`${styles.card} ${styles.revenue}`}>
          <div className={styles.cardHead}>
            <div><span className={styles.eyebrow}>Performance</span><h2>Revenue trend</h2></div>
            <span className={styles.badge}>৳12.84M total</span>
          </div>
          <div className={styles.chart} aria-label="Revenue trend from April to September">
            <div className={styles.gridLines}><span/><span/><span/><span/></div>
            <div className={styles.bars}>{months.map((item) => <div className={styles.barWrap} key={item.month}><div className={styles.bar} style={{ height: `${item.value}%` }} /><span>{item.month}</span></div>)}</div>
          </div>
        </article>

        <article className={styles.card}>
          <div className={styles.cardHead}><div><span className={styles.eyebrow}>Mix</span><h2>Sales by category</h2></div></div>
          <div className={styles.categoryList}>
            {["Technology", "Home & Living", "Lifestyle", "Fashion"].map((name, index) => <div className={styles.category} key={name}><div><span>{name}</span><b>{[42, 27, 19, 12][index]}%</b></div><div className={styles.track}><span style={{ width: `${[42, 27, 19, 12][index]}%` }}/></div></div>)}
          </div>
        </article>
      </section>

      <section className={styles.card}>
        <div className={styles.cardHead}><div><span className={styles.eyebrow}>Product intelligence</span><h2>Top products</h2></div><a href="#products">View catalogue →</a></div>
        <div className={styles.table} id="products">
          <div className={styles.tableRow + " " + styles.tableHeader}><span>Product</span><span>Revenue</span><span>Units sold</span></div>
          {products.map(([name, revenue, units], index) => <div className={styles.tableRow} key={name}><span><em>0{index + 1}</em>{name}</span><strong>{revenue}</strong><span>{units}</span></div>)}
        </div>
      </section>
    </main>
  );
}
