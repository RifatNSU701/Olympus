package com.olympus.api.catalog;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.UUID;

@Service
public class InventoryService {
    @Transactional
    public void validateAvailableStock(int availableStock, int requestedQuantity) {
        if (requestedQuantity <= 0) {
            throw new IllegalArgumentException("Quantity must be greater than zero");
        }
        if (requestedQuantity > availableStock) {
            throw new IllegalArgumentException("Insufficient stock");
        }
    }

    public UUID requireProductId(UUID productId) {
        if (productId == null) throw new IllegalArgumentException("Product ID is required");
        return productId;
    }
}
