package com.editorial.core.model.response;

import java.time.Instant;
import java.util.UUID;

public record PublicArticleResponse(
        UUID id,
        String title,
        String summary,
        String body,
        Instant publishedAt
) {
}
