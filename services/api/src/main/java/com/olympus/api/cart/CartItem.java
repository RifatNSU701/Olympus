package com.olympus.api.cart;

import com.olympus.api.catalog.Product;
import jakarta.persistence.*;
import java.util.UUID;

@Entity @Table(name="cart_items", uniqueConstraints=@UniqueConstraint(name="uk_cart_product", columnNames={"cart_id","product_id"}))
public class CartItem {
    @Id @GeneratedValue(strategy=GenerationType.UUID) private UUID id;
    @ManyToOne(fetch=FetchType.LAZY, optional=false) @JoinColumn(name="cart_id", nullable=false) private Cart cart;
    @ManyToOne(fetch=FetchType.LAZY, optional=false) @JoinColumn(name="product_id", nullable=false) private Product product;
    @Column(nullable=false) private int quantity;
    protected CartItem() {}
    public CartItem(Cart cart, Product product, int quantity){this.cart=cart;this.product=product;this.quantity=quantity;}
    public UUID getId(){return id;} public Cart getCart(){return cart;} public Product getProduct(){return product;}
    public int getQuantity(){return quantity;} public void setQuantity(int quantity){this.quantity=quantity;}
}