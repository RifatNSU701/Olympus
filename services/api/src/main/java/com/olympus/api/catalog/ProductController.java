package com.olympus.api.catalog;

import com.olympus.api.auth.UserRepository;
import jakarta.validation.Valid;
import jakarta.validation.constraints.*;
import org.springframework.data.domain.*;
import org.springframework.http.HttpStatus;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.*;
import java.math.BigDecimal;
import java.util.UUID;

@RestController @RequestMapping("/api/v1/products")
public class ProductController {
    private final ProductRepository products; private final CategoryRepository categories; private final UserRepository users;
    public ProductController(ProductRepository p,CategoryRepository c,UserRepository u){products=p;categories=c;users=u;}
    public record ProductResponse(UUID id,String name,String slug,String description,BigDecimal price,Integer stock,String status,UUID categoryId,UUID sellerId){}
    public record CreateRequest(@NotBlank @Size(max=240) String name,@Size(max=20000) String description,@NotNull @DecimalMin("0.00") BigDecimal price,@Min(0) Integer stock,UUID categoryId,@NotBlank @Size(max=260) String slug){}
    @GetMapping public Page<ProductResponse> list(@RequestParam(defaultValue="0") int page,@RequestParam(defaultValue="20") int size){
        if(size<1||size>100) throw new IllegalArgumentException("size must be between 1 and 100");
        return products.findByStatus(Product.Status.ACTIVE,PageRequest.of(page,size,Sort.by("createdAt").descending())).map(this::dto);
    }
    @GetMapping("/{id}") public ProductResponse get(@PathVariable UUID id){return dto(products.findById(id).orElseThrow());}
    @PostMapping @ResponseStatus(HttpStatus.CREATED)
    public ProductResponse create(@Valid @RequestBody CreateRequest r,Authentication auth){
        var user=users.findByEmailIgnoreCase(auth.getName()).orElseThrow();
        if(user.getRole()!=com.olympus.api.auth.Role.SELLER) throw new org.springframework.security.access.AccessDeniedException("Seller role required");
        if(products.existsBySlugIgnoreCase(r.slug())) throw new IllegalArgumentException("Product slug already exists");
        var p=new Product(); p.setSeller(user); p.setName(r.name().trim()); p.setSlug(r.slug().trim().toLowerCase()); p.setDescription(r.description()); p.setPrice(r.price()); p.setStock(r.stock()==null?0:r.stock());
        if(r.categoryId()!=null) p.setCategory(categories.findById(r.categoryId()).orElseThrow());
        p.setStatus(Product.Status.DRAFT); return dto(products.save(p));
    }
    private ProductResponse dto(Product p){return new ProductResponse(p.getId(),p.getName(),p.getSlug(),p.getDescription(),p.getPrice(),p.getStock(),p.getStatus().name(),p.getCategory()==null?null:p.getCategory().getId(),p.getSeller().getId());}
}
