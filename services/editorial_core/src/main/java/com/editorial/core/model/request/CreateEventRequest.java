package com.editorial.core.model.request;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;
import lombok.Getter;
import lombok.Setter;

import java.time.Instant;

@Getter
@Setter
public class CreateEventRequest {

    @NotBlank
    private String name;

    private String description;

    private String location;

    @NotNull
    private Instant startsAt;

    private Instant endsAt;
}