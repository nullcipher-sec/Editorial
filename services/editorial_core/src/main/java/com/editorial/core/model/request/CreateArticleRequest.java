package com.editorial.core.model.request;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class CreateArticleRequest {

    @NotBlank
    @Size(min = 5, max = 500)
    private String title;

    @Size(max = 2000)
    private String summary;

    @NotBlank
    @Size(min = 20)
    private String body;
}