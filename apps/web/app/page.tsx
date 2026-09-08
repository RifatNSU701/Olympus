export default function Home() {
  return (
    <main className="container">
      <nav className="nav">
        <div className="brand">OLYMPUS</div>
        <div className="navLinks">
          <a href="#shop">Shop</a>
          <a href="#categories">Categories</a>
          <a href="#sellers">Sellers</a>
          <a href="#about">About</a>
        </div>
      </nav>

      <section className="hero">
        <span className="eyebrow">The next generation marketplace</span>
        <h1>Commerce, elevated.</h1>
        <p>
          Olympus is being built as a premium multi-vendor marketplace where buyers,
          sellers and operators get a faster, safer and smarter commerce experience.
        </p>
        <div className="actions">
          <a className="button" href="#shop">Explore marketplace</a>
          <a className="button secondary" href="#sellers">Sell on Olympus</a>
        </div>
      </section>

      <section className="cards" aria-label="Olympus capabilities">
        <article className="card"><h2>Curated discovery</h2><p>Fast search, rich product pages and personalized recommendations designed around the customer.</p></article>
        <article className="card"><h2>Seller intelligence</h2><p>Inventory, orders, revenue and analytics in a focused operating dashboard.</p></article>
        <article className="card"><h2>Built for trust</h2><p>Secure authentication, auditable transactions and server-side authorization from day one.</p></article>
      </section>
    </main>
  );
}
