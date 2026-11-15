package com.editorial.core.model.request;

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;
import jakarta.validation.constraints.Size;
import lombok.Getter;
import lombok.Setter;

import java.util.UUID;

@Getter
@Setter
public class CreateCandidateRequest {

    @NotNull
    private UUID jobOpeningId;

    @NotBlank
    @Size(max = 255)
    private String fullName;

    @NotBlank
    @Email
    private String email;

    private String resumeSummary;
}