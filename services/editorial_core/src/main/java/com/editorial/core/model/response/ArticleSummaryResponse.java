package com.editorial.core.model.response;

import java.time.Instant;
import java.util.UUID;

public record ArticleSummaryResponse(UUID id, String title, String status, Instant publishedAt) {
}
