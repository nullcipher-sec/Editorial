package com.editorial.core.model.request;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class CreateCampaignRequest {

    @NotBlank
    @Size(max = 255)
    private String name;

    private String channel;
}