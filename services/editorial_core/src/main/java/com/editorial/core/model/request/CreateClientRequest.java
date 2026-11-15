package com.editorial.core.model.request;

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class CreateClientRequest {

    @NotBlank
    @Size(min = 3, max = 255)
    private String fullName;

    @Email
    private String email;

    @Size(max = 64)
    private String phone;
}