package com.olympus.api.cart;

import com.olympus.api.auth.User;
import jakarta.persistence.*;
import java.time.Instant;
import java.util.UUID;

@Entity @Table(name = "carts")
public class Cart {
    @Id @GeneratedValue(strategy = GenerationType.UUID) private UUID id;
    @OneToOne(fetch = FetchType.LAZY, optional = false) @JoinColumn(name = "buyer_id", nullable = false, unique = true) private User buyer;
    @Column(name = "created_at", nullable = false, updatable = false) private Instant createdAt;
    @Column(name = "updated_at", nullable = false) private Instant updatedAt;
    protected Cart() {}
    public Cart(User buyer) { this.buyer = buyer; }
    @PrePersist void create() { var now=Instant.now(); createdAt=now; updatedAt=now; }
    @PreUpdate void update() { updatedAt=Instant.now(); }
    public UUID getId(){return id;} public User getBuyer(){return buyer;}
}