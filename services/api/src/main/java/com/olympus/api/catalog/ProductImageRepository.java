package com.olympus.api.catalog;

import org.springframework.data.jpa.repository.JpaRepository;
import java.util.*;

public interface ProductImageRepository extends JpaRepository<ProductImage, UUID> {
    List<ProductImage> findByProductIdOrderBySortOrderAsc(UUID productId);
}
