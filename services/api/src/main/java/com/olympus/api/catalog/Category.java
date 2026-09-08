package com.olympus.api.catalog;

import jakarta.persistence.*;
import java.util.UUID;

@Entity
@Table(name = "categories")
public class Category {
    @Id @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;
    @Column(nullable = false, unique = true, length = 120) private String name;
    @Column(nullable = false, unique = true, length = 140) private String slug;
    public UUID getId(){return id;} public String getName(){return name;} public void setName(String name){this.name=name;}
    public String getSlug(){return slug;} public void setSlug(String slug){this.slug=slug;}
}
