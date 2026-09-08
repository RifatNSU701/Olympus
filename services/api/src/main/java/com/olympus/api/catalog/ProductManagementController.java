package com.olympus.api.catalog;

import com.olympus.api.auth.UserRepository;
import jakarta.validation.Valid;
import jakarta.validation.constraints.*;
import org.springframework.http.HttpStatus;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.*;

import java.math.BigDecimal;
import java.util.*;

@RestController
@RequestMapping("/api/v1/seller/products")
public class ProductManagementController {
    private final ProductRepository products;
    private final CategoryRepository categories;
    private final ProductImageRepository images;
    private final UserRepository users;

    public ProductManagementController(ProductRepository products, CategoryRepository categories, ProductImageRepository images, UserRepository users) {
        this.products=products; this.categories=categories; this.images=images; this.users=users;
    }

    public record UpdateRequest(@NotBlank @Size(max=240) String name, @Size(max=20000) String description,
                                @NotNull @DecimalMin("0.00") BigDecimal price, @Min(0) int stock,
                                UUID categoryId, @NotBlank @Size(max=260) String slug) {}
    public record ImageRequest(@NotBlank @Size(max=2048) String imageUrl, @Min(0) int sortOrder, boolean primary) {}

    @GetMapping
    public List<Product> mine(Authentication auth) {
        var user=users.findByEmailIgnoreCase(auth.getName()).orElseThrow();
        return products.findBySellerId(user.getId(), org.springframework.data.domain.Pageable.unpaged()).getContent();
    }

    @PutMapping("/{id}")
    public Product update(@PathVariable UUID id, @Valid @RequestBody UpdateRequest r, Authentication auth) {
        var user=users.findByEmailIgnoreCase(auth.getName()).orElseThrow();
        var p=products.findById(id).orElseThrow();
        if(!p.getSeller().getId().equals(user.getId())) throw new AccessDeniedException("Product ownership required");
        if(!p.getSlug().equalsIgnoreCase(r.slug()) && products.existsBySlugIgnoreCase(r.slug())) throw new IllegalArgumentException("Product slug already exists");
        p.setName(r.name().trim()); p.setSlug(r.slug().trim().toLowerCase()); p.setDescription(r.description()); p.setPrice(r.price()); p.setStock(r.stock());
        if(r.categoryId()!=null) p.setCategory(categories.findById(r.categoryId()).orElseThrow());
        return products.save(p);
    }

    @PostMapping("/{id}/images")
    @ResponseStatus(HttpStatus.CREATED)
    public ProductImage addImage(@PathVariable UUID id, @Valid @RequestBody ImageRequest r, Authentication auth) {
        var user=users.findByEmailIgnoreCase(auth.getName()).orElseThrow();
        var p=products.findById(id).orElseThrow();
        if(!p.getSeller().getId().equals(user.getId())) throw new AccessDeniedException("Product ownership required");
        return images.save(new ProductImage(p, r.imageUrl().trim(), r.sortOrder(), r.primary()));
    }

    @GetMapping("/{id}/images")
    public List<ProductImage> getImages(@PathVariable UUID id) { return images.findByProductIdOrderBySortOrderAsc(id); }
}
