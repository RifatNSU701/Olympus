package com.olympus.api.cart;

import jakarta.validation.constraints.Min;
import org.springframework.web.bind.annotation.*;
import org.springframework.security.core.Authentication;
import java.util.UUID;

@RestController @RequestMapping("/api/v1/cart")
public class CartController {
    private final CartService service;
    public CartController(CartService service){this.service=service;}
    public record ItemRequest(UUID productId,@Min(1) int quantity){}
    @GetMapping public CartService.CartView get(Authentication auth){return service.view(auth.getName());}
    @PostMapping("/items") public CartService.CartView add(@RequestBody ItemRequest r, Authentication auth){return service.add(auth.getName(),r.productId(),r.quantity());}
    @PutMapping("/items/{productId}") public CartService.CartView update(@PathVariable UUID productId,@RequestBody ItemRequest r,Authentication auth){return service.update(auth.getName(),productId,r.quantity());}
    @DeleteMapping("/items/{productId}") public CartService.CartView remove(@PathVariable UUID productId,Authentication auth){return service.remove(auth.getName(),productId);}
}