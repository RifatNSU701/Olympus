package com.olympus.api.cart;

import org.springframework.data.jpa.repository.JpaRepository;
import java.util.*;

public interface CartItemRepository extends JpaRepository<CartItem, UUID> {
    Optional<CartItem> findByCartIdAndProductId(UUID cartId, UUID productId);
    List<CartItem> findAllByCartId(UUID cartId);
    void deleteByCartIdAndProductId(UUID cartId, UUID productId);
}