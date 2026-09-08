package com.olympus.api.catalog;

import jakarta.validation.constraints.Min;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.*;

import java.util.Map;
import java.util.UUID;

@RestController
@RequestMapping("/api/v1/inventory")
public class InventoryController {
    private final InventoryService inventoryService;

    public InventoryController(InventoryService inventoryService) {
        this.inventoryService = inventoryService;
    }

    @PostMapping("/validate")
    @PreAuthorize("hasAnyRole('BUYER','SELLER','ADMIN','SUPER_ADMIN')")
    public Map<String, Object> validate(@RequestParam UUID productId,
                                        @RequestParam @Min(1) int availableStock,
                                        @RequestParam @Min(1) int quantity) {
        inventoryService.requireProductId(productId);
        inventoryService.validateAvailableStock(availableStock, quantity);
        return Map.of("productId", productId, "available", true, "quantity", quantity);
    }
}
