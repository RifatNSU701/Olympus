package com.olympus.api.user;

import com.olympus.api.auth.UserRepository;
import jakarta.validation.Valid;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.*;
import java.util.UUID;

@RestController
@RequestMapping("/api/v1/users/me")
public class UserProfileController {
    private final UserRepository users;
    public UserProfileController(UserRepository users){this.users=users;}

    public record ProfileResponse(UUID id,String email,String fullName,String phone,String role,String status){}
    public record UpdateRequest(@NotBlank @Size(max=160) String fullName,@Size(max=32) String phone){}

    @GetMapping
    public ProfileResponse get(Authentication auth){return dto(current(auth));}

    @PutMapping
    public ProfileResponse update(@Valid @RequestBody UpdateRequest request,Authentication auth){
        var user=current(auth);
        user.setFullName(request.fullName().trim());
        user.setPhone(request.phone()==null?null:request.phone().trim());
        return dto(users.save(user));
    }

    private com.olympus.api.auth.User current(Authentication auth){
        return users.findByEmailIgnoreCase(auth.getName()).orElseThrow();
    }
    private ProfileResponse dto(com.olympus.api.auth.User u){
        return new ProfileResponse(u.getId(),u.getEmail(),u.getFullName(),u.getPhone(),u.getRole().name(),u.getStatus().name());
    }
}
