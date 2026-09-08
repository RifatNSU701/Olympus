package com.olympus.api.catalog;

import org.springframework.data.domain.*;
import org.springframework.data.jpa.repository.JpaRepository;
import java.util.UUID;

public interface ProductRepository extends JpaRepository<Product, UUID> {
    Page<Product> findByStatus(Product.Status status, Pageable pageable);
    Page<Product> findBySellerId(UUID sellerId, Pageable pageable);
    boolean existsBySlugIgnoreCase(String slug);
}
