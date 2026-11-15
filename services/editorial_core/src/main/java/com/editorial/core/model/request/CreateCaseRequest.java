package com.editorial.core.model.request;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;
import jakarta.validation.constraints.Size;
import lombok.Getter;
import lombok.Setter;

import java.util.UUID;

@Getter
@Setter
public class CreateCaseRequest {

    @NotNull
    private UUID clientId;

    @NotBlank
    @Size(min = 5, max = 500)
    private String subject;

    @NotBlank
    @Size(min = 10, max = 2000)
    private String description;
}