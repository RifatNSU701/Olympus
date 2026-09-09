import { useEffect, useMemo, useState } from 'react';
import { Search, SlidersHorizontal, ShoppingBag, ArrowUpRight } from 'lucide-react';
import { ProductDetail } from './product';

type Product = { id: string; name: string; slug: string; description?: string; price: number; stock: number; status: string };
const API = import.meta.env.VITE_API_URL ?? 'http://localhost:8080';

export function Catalog() {
  const [products, setProducts] = useState<Product[]>([]); const [query,setQuery]=useState(''); const [selected,setSelected]=useState<string|null>(null); const [loading,setLoading]=useState(true); const [error,setError]=useState('');
  useEffect(()=>{fetch(`${API}/api/v1/products?limit=48`).then(r=>{if(!r.ok)throw new Error('Unable to load products');return r.json()}).then(setProducts).catch(e=>setError(e.message)).finally(()=>setLoading(false))},[]);
  const filtered=useMemo(()=>products.filter(p=>`${p.name} ${p.description??''}`.toLowerCase().includes(query.toLowerCase())),[products,query]);
  if(selected)return <ProductDetail id={selected} onBack={()=>setSelected(null)}/>;
  return <section className="catalog" id="shop"><header className="catalog-head"><div><span className="eyebrow">THE COLLECTION</span><h2>Explore products.</h2></div><div className="catalog-tools"><label><Search size={17}/><input value={query} onChange={e=>setQuery(e.target.value)} placeholder="Search products" /></label><button className="filter"><SlidersHorizontal size={17}/> Filter</button></div></header>{loading&&<div className="catalog-state">Loading the collection…</div>}{error&&<div className="catalog-state">{error}</div>}{!loading&&!error&&<div className="grid">{filtered.map(product=><article className="card" key={product.id} onClick={()=>setSelected(product.id)}><div className="card-visual"><span>{product.status}</span><button aria-label={`View ${product.name}`} onClick={e=>{e.stopPropagation();setSelected(product.id)}}><ShoppingBag size={18}/></button></div><div className="card-meta"><div><h3>{product.name}</h3><p>{product.description||'Discover this Olympus product.'}</p></div><strong>৳{Number(product.price).toLocaleString('en-BD')}</strong></div><div className="stock">{product.stock>0?`${product.stock} available`:'Out of stock'} <ArrowUpRight size={14}/></div></article>)}</div>}{!loading&&!error&&filtered.length===0&&<div className="catalog-state">No products match your search.</div>}</section>;
}
