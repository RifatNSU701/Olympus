import { useCallback, useEffect, useState } from 'react';
import { ArrowLeft, Minus, Plus, ShoppingBag, Trash2, ArrowRight } from 'lucide-react';
import './cart.css';

type Item={id:string;product_id:string;product_name:string;unit_price:number;available_stock:number;quantity:number;line_total:number};
type Cart={id:string;items:Item[];subtotal:number};
const API=import.meta.env.VITE_API_URL??'http://localhost:8080';

export function Cart({onBack,onCheckout}:{onBack:()=>void;onCheckout:()=>void}){
 const [cart,setCart]=useState<Cart|null>(null);const [error,setError]=useState('');const token=localStorage.getItem('olympus_token');
 const load=useCallback(async()=>{if(!token){setError('Sign in to view your cart.');return}try{const r=await fetch(`${API}/api/v1/cart`,{headers:{Authorization:`Bearer ${token}`}});if(!r.ok)throw new Error('Unable to load cart');setCart(await r.json())}catch(e){setError((e as Error).message)}},[token]);
 useEffect(()=>{load()},[load]);
 async function update(id:string,quantity:number){if(quantity<1)return remove(id);const r=await fetch(`${API}/api/v1/cart/items/${id}`,{method:'PUT',headers:{'Content-Type':'application/json',Authorization:`Bearer ${token}`},body:JSON.stringify({quantity})});if(!r.ok){setError('Quantity exceeds available stock.');return}setCart(await r.json())}
 async function remove(id:string){const r=await fetch(`${API}/api/v1/cart/items/${id}`,{method:'DELETE',headers:{Authorization:`Bearer ${token}`}});if(r.ok)setCart(await r.json())}
 if(!token||error&&!cart)return <section className="cart-page"><button className="back" onClick={onBack}><ArrowLeft size={17}/> Continue shopping</button><div className="cart-empty"><ShoppingBag size={32}/><h2>{error||'Your cart is empty.'}</h2><p>Sign in to manage your Olympus cart.</p></div></section>;
 if(!cart)return <div className="cart-empty">Loading cart…</div>;
 return <section className="cart-page"><button className="back" onClick={onBack}><ArrowLeft size={17}/> Continue shopping</button><div className="cart-heading"><span className="eyebrow">YOUR SELECTION</span><h1>Your cart.</h1></div>{cart.items.length===0?<div className="cart-empty"><ShoppingBag size={32}/><h2>Your cart is empty.</h2><button className="primary" onClick={onBack}>Explore products <ArrowRight size={17}/></button></div>:<div className="cart-layout"><div className="cart-items">{cart.items.map(i=><article className="cart-item" key={i.id}><div className="item-art"/><div className="item-info"><h3>{i.product_name}</h3><p>৳{Number(i.unit_price).toLocaleString('en-BD')} each</p><div className="qty"><button onClick={()=>update(i.id,i.quantity-1)}><Minus size={14}/></button><b>{i.quantity}</b><button disabled={i.quantity>=i.available_stock} onClick={()=>update(i.id,i.quantity+1)}><Plus size={14}/></button></div></div><strong>৳{Number(i.line_total).toLocaleString('en-BD')}</strong><button className="remove" aria-label="Remove item" onClick={()=>remove(i.id)}><Trash2 size={17}/></button></article>)}</div><aside className="summary"><span className="eyebrow">ORDER SUMMARY</span><div><span>Subtotal</span><b>৳{Number(cart.subtotal).toLocaleString('en-BD')}</b></div><div><span>Shipping</span><span>Calculated at checkout</span></div><hr/><div className="total"><span>Total</span><b>৳{Number(cart.subtotal).toLocaleString('en-BD')}</b></div><button className="checkout" onClick={onCheckout}>Proceed to checkout <ArrowRight size={17}/></button></aside></div>}</section>;
}
