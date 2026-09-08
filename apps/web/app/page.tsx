const products = [
  { name: "Apex Wireless Headphones", category: "Audio", price: "৳8,490", tag: "New" },
  { name: "Vertex Mechanical Keyboard", category: "Workspace", price: "৳12,990", tag: "Popular" },
  { name: "Atlas Everyday Backpack", category: "Lifestyle", price: "৳5,990", tag: "Featured" },
  { name: "Nova Smart Lamp", category: "Home", price: "৳4,290", tag: "Limited" },
];

export default function Home() {
  return (
    <main className="site-shell">
      <nav className="nav container">
        <div className="brand"><span>O</span> OLYMPUS</div>
        <div className="navLinks"><a href="#shop">Shop</a><a href="#categories">Categories</a><a href="#sellers">Sellers</a></div>
        <div className="navActions"><button aria-label="Search">⌕</button><button aria-label="Shopping bag">Bag <b>0</b></button></div>
      </nav>

      <section className="hero container">
        <div className="heroCopy"><span className="eyebrow">The elevated marketplace</span><h1>Find what<br/><em>matters.</em></h1><p>Discover exceptional products from independent sellers, brought together in one refined shopping experience.</p><div className="actions"><a className="button" href="#shop">Explore marketplace <span>↗</span></a><a className="textLink" href="#sellers">Sell on Olympus</a></div></div>
        <div className="heroVisual" aria-label="Featured Olympus collection"><div className="orb orbOne"/><div className="orb orbTwo"/><div className="visualLabel">OLYMPUS / 001</div><div className="visualProduct"><span>01</span><strong>OBJECTS<br/>WORTH<br/>KEEPING.</strong></div></div>
      </section>

      <section id="shop" className="section container"><div className="sectionHead"><div><span className="eyebrow">Curated for you</span><h2>Featured products</h2></div><a className="textLink" href="#shop">View all ↗</a></div><div className="productGrid">{products.map((product) => <article className="productCard" key={product.name}><div className="productArt"><span className="productTag">{product.tag}</span><span className="productIndex">01 / 04</span><div className="artShape"/></div><div className="productInfo"><div><span>{product.category}</span><h3>{product.name}</h3></div><strong>{product.price}</strong></div></article>)}</div></section>

      <section id="categories" className="categories container"><span className="eyebrow">Explore</span><h2>Everything you need.<br/><em>Nothing you don't.</em></h2><div className="categoryLinks"><a href="#shop">Technology <span>→</span></a><a href="#shop">Home & Living <span>→</span></a><a href="#shop">Fashion <span>→</span></a><a href="#shop">Lifestyle <span>→</span></a></div></section>

      <footer id="about" className="footer container"><div><div className="brand"><span>O</span> OLYMPUS</div><p>Commerce, elevated.</p></div><div><span className="eyebrow">Built for buyers & sellers</span><p>Premium discovery. Secure transactions. Smarter commerce.</p></div></footer>
    </main>
  );
}
