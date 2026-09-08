package com.olympus.api.catalog;

import jakarta.persistence.*;
import java.util.UUID;

@Entity
@Table(name = "product_images")
public class ProductImage {
    @Id @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "product_id", nullable = false)
    private Product product;
    @Column(name = "image_url", nullable = false, length = 2048)
    private String imageUrl;
    @Column(name = "sort_order", nullable = false)
    private Integer sortOrder = 0;
    @Column(name = "is_primary", nullable = false)
    private boolean primary;
    protected ProductImage() {}
    public ProductImage(Product product, String imageUrl, Integer sortOrder, boolean primary) { this.product=product; this.imageUrl=imageUrl; this.sortOrder=sortOrder; this.primary=primary; }
    public UUID getId(){return id;} public Product getProduct(){return product;} public String getImageUrl(){return imageUrl;} public Integer getSortOrder(){return sortOrder;} public boolean isPrimary(){return primary;}
}
