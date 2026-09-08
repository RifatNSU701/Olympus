package com.olympus.api.catalog;

import jakarta.validation.Valid;
import jakarta.validation.constraints.NotBlank;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;
import java.util.List;
import java.util.UUID;

@RestController @RequestMapping("/api/v1/categories")
public class CategoryController {
    private final CategoryRepository repo;
    public CategoryController(CategoryRepository repo){this.repo=repo;}
    @GetMapping public List<Category> list(){return repo.findAll();}
    public record CreateRequest(@NotBlank String name,@NotBlank String slug){}
    @PostMapping @ResponseStatus(HttpStatus.CREATED)
    public Category create(@Valid @RequestBody CreateRequest r){
        if(repo.existsBySlugIgnoreCase(r.slug())) throw new IllegalArgumentException("Category slug already exists");
        var c=new Category(); c.setName(r.name().trim()); c.setSlug(r.slug().trim().toLowerCase()); return repo.save(c);
    }
}
