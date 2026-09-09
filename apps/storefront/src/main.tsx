import React from 'react';
import { createRoot } from 'react-dom/client';
import { ArrowRight, Search, ShoppingBag, ShieldCheck, Sparkles, Truck } from 'lucide-react';
import { Catalog } from './catalog';
import './styles.css';
import './catalog.css';

const highlights = [
  { icon: ShieldCheck, title: 'Trusted commerce', text: 'Secure accounts, protected payments and transparent order tracking.' },
  { icon: Truck, title: 'Fast fulfillment', text: 'A marketplace experience designed around reliable delivery.' },
  { icon: Sparkles, title: 'Curated quality', text: 'Discover products from verified marketplace sellers.' },
];

function App() {
  return <div className="app">
    <nav className="nav"><div className="brand">OLYMPUS<span>®</span></div><div className="navlinks"><a href="#shop">Shop</a><a href="#sellers">Sellers</a><a href="#about">About</a></div><div className="actions"><button className="icon"><Search size={19}/></button><button className="bag"><ShoppingBag size={19}/><b>0</b></button></div></nav>
    <main>
      <section className="hero"><div className="eyebrow">THE MODERN MARKETPLACE</div><h1>Commerce,<br/><em>elevated.</em></h1><p>Discover exceptional products from ambitious sellers, all in one beautifully simple marketplace.</p><div className="hero-actions"><a className="primary" href="#shop">Explore products <ArrowRight size={18}/></a><button className="secondary">Become a seller</button></div><div className="orb orb-a"/><div className="orb orb-b"/></section>
      <Catalog />
      <section className="features" id="about">{highlights.map(({icon: Icon,title,text}) => <article key={title}><Icon size={22}/><h3>{title}</h3><p>{text}</p></article>)}</section>
      <section className="discover"><div><span className="eyebrow">DISCOVER OLYMPUS</span><h2>Built for people<br/>who expect more.</h2></div><button className="round"><ArrowRight/></button></section>
    </main>
    <footer><div className="brand">OLYMPUS<span>®</span></div><small>© 2026 Olympus Marketplace</small></footer>
  </div>;
}
createRoot(document.getElementById('root')!).render(<React.StrictMode><App /></React.StrictMode>);
