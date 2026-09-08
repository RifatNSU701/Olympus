package com.olympus.api.cart;

import com.olympus.api.auth.UserRepository;
import com.olympus.api.catalog.Product;
import com.olympus.api.catalog.ProductRepository;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.math.BigDecimal;
import java.util.*;

@Service
public class CartService {
    private final CartRepository carts; private final CartItemRepository items; private final ProductRepository products; private final UserRepository users;
    public CartService(CartRepository carts, CartItemRepository items, ProductRepository products, UserRepository users){this.carts=carts;this.items=items;this.products=products;this.users=users;}
    private Cart cart(String email){var u=users.findByEmailIgnoreCase(email).orElseThrow(); return carts.findByBuyerId(u.getId()).orElseGet(()->carts.save(new Cart(u)));}
    @Transactional(readOnly=true) public CartView view(String email){var c=cart(email); var rows=items.findAllByCartId(c.getId()).stream().map(i->new CartRow(i.getId(),i.getProduct().getId(),i.getProduct().getName(),i.getProduct().getPrice(),i.getQuantity(),i.getProduct().getPrice().multiply(BigDecimal.valueOf(i.getQuantity())))).toList(); var total=rows.stream().map(CartRow::lineTotal).reduce(BigDecimal.ZERO,BigDecimal::add); return new CartView(c.getId(),rows,total);}
    @Transactional public CartView add(String email, UUID productId, int quantity){if(quantity<1)throw new IllegalArgumentException("Quantity must be at least 1"); var c=cart(email); var p=products.findById(productId).orElseThrow(); if(p.getStatus()!= Product.Status.ACTIVE)throw new IllegalArgumentException("Product is not available"); var existing=items.findByCartIdAndProductId(c.getId(),productId); int next=existing.map(i->i.getQuantity()+quantity).orElse(quantity); if(next>p.getStock())throw new IllegalArgumentException("Insufficient stock"); if(existing.isPresent())existing.get().setQuantity(next); else items.save(new CartItem(c,p,quantity)); return view(email);}
    @Transactional public CartView update(String email, UUID productId, int quantity){if(quantity<1)throw new IllegalArgumentException("Quantity must be at least 1"); var c=cart(email); var i=items.findByCartIdAndProductId(c.getId(),productId).orElseThrow(); if(quantity>i.getProduct().getStock())throw new IllegalArgumentException("Insufficient stock"); i.setQuantity(quantity); return view(email);}
    @Transactional public CartView remove(String email, UUID productId){var c=cart(email); items.deleteByCartIdAndProductId(c.getId(),productId); return view(email);}
    public record CartRow(UUID id,UUID productId,String name,BigDecimal unitPrice,int quantity,BigDecimal lineTotal){}
    public record CartView(UUID id,List<CartRow> items,BigDecimal total){}
}