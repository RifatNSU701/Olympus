import React, { useEffect, useState } from 'react';
import { ArrowRight, Search, ShoppingBag, ShieldCheck, Sparkles, Truck } from 'lucide-react';
import { AuthDialog, AccountButton } from './auth';
import { Catalog } from './catalog';
import './styles.css';
import './catalog.css';

const API = import.meta.env.VITE_API_URL ?? 'http://localhost:8080';
const highlights = [
  { icon: ShieldCheck, title: 'Trusted commerce', text: 'Secure accounts, protected payments and transparent order tracking.' },
  { icon: Truck, title: 'Fast fulfillment', text: 'A marketplace experience designed around reliable delivery.' },
  { icon: Sparkles, title: 'Curated quality', text: 'Discover products from verified marketplace sellers.' },
];
type User = { id: string; email: string; role: string; status: string; full_name: string };
type CartItem = { id: string; product_id: string; product_name: string; unit_price: number; quantity: number; line_total: number };
type Cart = { items: CartItem[]; subtotal: number };

function CartDrawer({ open, onClose, onSignIn }: { open: boolean; onClose: () => void; onSignIn: () => void }) {
  const [cart, setCart] = useState<Cart | null>(null);
  const [message, setMessage] = useState('');
  useEffect(() => {
    if (!open) return;
    const token = localStorage.getItem('olympus_token');
    if (!token) { setCart(null); setMessage('Sign in to view your cart.'); return; }
    setMessage('Loading cart…');
    fetch(`${API}/api/v1/cart`, { headers: { Authorization: `Bearer ${token}` } })
      .then(r => { if (!r.ok) throw new Error(r.status === 401 || r.status === 403 ? 'Your session is not active.' : 'Unable to load cart.'); return r.json(); })
      .then(data => { setCart(data); setMessage(''); })
      .catch(e => setMessage(e.message));
  }, [open]);
  if (!open) return null;
  return <div className="drawer-backdrop" onClick={onClose}><aside className="cart-drawer" onClick={e => e.stopPropagation()}>
    <header><div><span className="eyebrow">YOUR CART</span><h2>Ready to checkout.</h2></div><button className="drawer-close" aria-label="Close cart" onClick={onClose}>×</button></header>
    {message && <p className="drawer-message">{message}{!cart && <><br/><button className="drawer-signin" onClick={() => { onClose(); onSignIn(); }}>Sign in or create an account</button></>}</p>}
    {cart && cart.items.length === 0 && <p className="drawer-message">Your cart is empty. Explore the collection to add something.</p>}
    {cart && cart.items.length > 0 && <><div className="cart-items">{cart.items.map(item => <div className="cart-item" key={item.id}><div><strong>{item.product_name}</strong><small>{item.quantity} × ৳{Number(item.unit_price).toLocaleString('en-BD')}</small></div><b>৳{Number(item.line_total).toLocaleString('en-BD')}</b></div>)}</div><div className="cart-total"><span>Subtotal</span><strong>৳{Number(cart.subtotal).toLocaleString('en-BD')}</strong></div><button className="checkout-button" onClick={() => setMessage('Checkout is ready for the authenticated account.')}>Continue to checkout <ArrowRight size={17}/></button></>}
  </aside></div>;
}

function App() {
  const [cartOpen, setCartOpen] = useState(false);
  const [authOpen, setAuthOpen] = useState(false);
  const [user, setUser] = useState<User | null>(null);
  useEffect(() => {
    const token = localStorage.getItem('olympus_token');
    if (!token) return;
    fetch(`${API}/api/v1/auth/me`, { headers: { Authorization: `Bearer ${token}` } }).then(r => r.ok ? r.json() : Promise.reject()).then(setUser).catch(() => { localStorage.removeItem('olympus_token'); setUser(null); });
  }, []);
  function logout() { localStorage.removeItem('olympus_token'); setUser(null); }
  return <div className="app">
    <nav className="nav"><div className="brand">OLYMPUS<span>®</span></div><div className="navlinks"><a href="#shop">Shop</a><a href="#sellers">Sellers</a><a href="#about">About</a></div><div className="actions"><button className="icon" aria-label="Search"><Search size={19}/></button><button className="bag" aria-label="Open cart" onClick={() => setCartOpen(true)}><ShoppingBag size={19}/><b>Cart</b></button><AccountButton user={user} onClick={() => setAuthOpen(true)} onLogout={logout}/></div></nav>
    <main><section className="hero"><div className="eyebrow">THE MODERN MARKETPLACE</div><h1>Commerce,<br/><em>elevated.</em></h1><p>Discover exceptional products from ambitious sellers, all in one beautifully simple marketplace.</p><div className="hero-actions"><a className="primary" href="#shop">Explore products <ArrowRight size={18}/></a><button className="secondary" onClick={() => user ? setCartOpen(true) : setAuthOpen(true)}>{user ? 'View your cart' : 'Sign in to shop'}</button></div><div className="orb orb-a"/><div className="orb orb-b"/></section><Catalog/><section className="features" id="about">{highlights.map(({icon: Icon,title,text}) => <article key={title}><Icon size={22}/><h3>{title}</h3><p>{text}</p></article>)}</section><section className="discover"><div><span className="eyebrow">DISCOVER OLYMPUS</span><h2>Built for people<br/>who expect more.</h2></div><button className="round" aria-label="Open cart" onClick={() => setCartOpen(true)}><ArrowRight/></button></section></main>
    <footer><div className="brand">OLYMPUS<span>®</span></div><small>© 2026 Olympus Marketplace</small></footer>
    <CartDrawer open={cartOpen} onClose={() => setCartOpen(false)} onSignIn={() => setAuthOpen(true)}/><AuthDialog open={authOpen} onClose={() => setAuthOpen(false)} onAuthenticated={setUser}/>
  </div>;
}
createRoot(document.getElementById('root')!).render(<React.StrictMode><App /></React.StrictMode>);
