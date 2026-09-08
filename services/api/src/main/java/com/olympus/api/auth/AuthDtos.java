package com.olympus.api.auth;

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import java.util.UUID;

public final class AuthDtos {
    private AuthDtos() {}

    public record RegisterRequest(
            @NotBlank @Size(max = 160) String fullName,
            @NotBlank @Email @Size(max = 320) String email,
            @NotBlank @Size(min = 8, max = 128) String password,
            @Size(max = 32) String phone,
            Role role) {}

    public record LoginRequest(
            @NotBlank @Email @Size(max = 320) String email,
            @NotBlank @Size(max = 128) String password) {}

    public record AuthResponse(String accessToken, String tokenType, long expiresInSeconds,
                               UUID userId, String email, String fullName, Role role) {}

    public record MeResponse(UUID userId, String email, String fullName, String phone, Role role,
                             UserStatus status) {}
}
