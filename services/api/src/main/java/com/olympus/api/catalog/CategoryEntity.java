package com.olympus.api.catalog;

import jakarta.persistence.*;
import java.util.UUID;

@Entity
@Table(name = "categories")
public class CategoryEntity {
    @Id @GeneratedValue(strategy = GenerationType.UUID) private UUID id;
    @Column(nullable = false, unique = true, length = 120) private String name;
    @Column(nullable = false, unique = true, length = 140) private String slug;
    protected CategoryEntity() {}
    public CategoryEntity(String name, String slug) { this.name = name; this.slug = slug; }
    public UUID getId() { return id; }
    public String getName() { return name; }
    public String getSlug() { return slug; }
}