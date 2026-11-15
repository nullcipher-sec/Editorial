package com.editorial.core.model.request;

import com.fasterxml.jackson.annotation.JsonAlias;
import jakarta.validation.constraints.NotBlank;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class LoginRequest {

    @NotBlank
    @JsonAlias({"username", "user"})
    private String username;

    @NotBlank
    private String password;
}