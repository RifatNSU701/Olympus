package com.olympus.api.catalog;

import com.olympus.api.auth.User;
import jakarta.persistence.*;
import java.math.BigDecimal;
import java.time.Instant;
import java.util.UUID;

@Entity @Table(name="products")
public class Product {
    public enum Status { DRAFT, ACTIVE, ARCHIVED }
    @Id @GeneratedValue(strategy=GenerationType.UUID) private UUID id;
    @ManyToOne(fetch=FetchType.LAZY, optional=false) @JoinColumn(name="seller_id", nullable=false) private User seller;
    @ManyToOne(fetch=FetchType.LAZY) @JoinColumn(name="category_id") private Category category;
    @Column(nullable=false,length=240) private String name;
    @Column(nullable=false,unique=true,length=260) private String slug;
    @Column(columnDefinition="TEXT") private String description;
    @Column(nullable=false,precision=19,scale=4) private BigDecimal price;
    @Column(nullable=false) private Integer stock=0;
    @Enumerated(EnumType.STRING) @Column(nullable=false,length=32) private Status status=Status.DRAFT;
    @Column(name="created_at",nullable=false,updatable=false) private Instant createdAt;
    @Column(name="updated_at",nullable=false) private Instant updatedAt;
    @PrePersist void create(){var n=Instant.now();createdAt=n;updatedAt=n;}
    @PreUpdate void update(){updatedAt=Instant.now();}
    public UUID getId(){return id;} public User getSeller(){return seller;} public void setSeller(User v){seller=v;}
    public Category getCategory(){return category;} public void setCategory(Category v){category=v;} public String getName(){return name;} public void setName(String v){name=v;}
    public String getSlug(){return slug;} public void setSlug(String v){slug=v;} public String getDescription(){return description;} public void setDescription(String v){description=v;}
    public BigDecimal getPrice(){return price;} public void setPrice(BigDecimal v){price=v;} public Integer getStock(){return stock;} public void setStock(Integer v){stock=v;}
    public Status getStatus(){return status;} public void setStatus(Status v){status=v;} public Instant getCreatedAt(){return createdAt;} public Instant getUpdatedAt(){return updatedAt;}
}
