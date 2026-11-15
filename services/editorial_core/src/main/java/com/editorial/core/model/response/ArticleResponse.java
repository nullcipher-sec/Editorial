package com.editorial.core.model.response;

import com.editorial.core.entity.ArticleStatus;
import com.fasterxml.jackson.annotation.JsonProperty;

import java.time.Instant;
import java.util.UUID;

public record ArticleResponse(
        UUID id,
        String title,
        String summary,
        String body,
        ArticleStatus status,
        @JsonProperty("authorId") UUID authorId,
        @JsonProperty("publishedAt") Instant publishedAt
) {
}