import { useEffect, useState } from 'react';
import { ArrowLeft, Minus, Plus, ShoppingBag } from 'lucide-react';

type Product={id:string;name:string;slug:string;description?:string;price:number;stock:number;status:string};
const API=import.meta.env.VITE_API_URL??'http://localhost:8080';

export function ProductDetail({id,onBack}:{id:string;onBack:()=>void}){
 const [product,setProduct]=useState<Product|null>(null); const [qty,setQty]=useState(1); const [message,setMessage]=useState(''); const [loading,setLoading]=useState(true);
 useEffect(()=>{fetch(`${API}/api/v1/products/${id}`).then(r=>{if(!r.ok)throw new Error('Product unavailable');return r.json()}).then(setProduct).catch(e=>setMessage(e.message)).finally(()=>setLoading(false))},[id]);
 async function add(){const token=localStorage.getItem('olympus_token');if(!token){setMessage('Sign in to add items to your cart.');return}setMessage('Adding…');try{const r=await fetch(`${API}/api/v1/cart/items`,{method:'POST',headers:{'Content-Type':'application/json',Authorization:`Bearer ${token}`},body:JSON.stringify({product_id:id,quantity:qty})});if(!r.ok)throw new Error('Unable to add to cart');setMessage('Added to cart.')}catch(e){setMessage((e as Error).message)}}
 if(loading)return <div className="detail-state">Loading product…</div>; if(!product)return <div className="detail-state"><button onClick={onBack}><ArrowLeft/> Back</button><p>{message}</p></div>;
 return <section className="detail"><button className="back" onClick={onBack}><ArrowLeft size={17}/> Back to collection</button><div className="detail-grid"><div className="detail-visual"><span>{product.status}</span></div><div className="detail-info"><span className="eyebrow">OLYMPUS COLLECTION</span><h1>{product.name}</h1><strong>৳{Number(product.price).toLocaleString('en-BD')}</strong><p>{product.description||'A carefully selected product from the Olympus marketplace.'}</p><div className="quantity"><button disabled={qty<=1} onClick={()=>setQty(q=>q-1)}><Minus size={16}/></button><b>{qty}</b><button disabled={qty>=product.stock} onClick={()=>setQty(q=>q+1)}><Plus size={16}/></button></div><button className="add" disabled={!product.stock} onClick={add}><ShoppingBag size={18}/>{product.stock?'Add to cart':'Out of stock'}</button>{message&&<small className="message">{message}</small>}</div></div></section>
}
