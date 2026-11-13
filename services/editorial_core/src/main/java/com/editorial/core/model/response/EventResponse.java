package com.editorial.core.model.response;

import java.time.Instant;
import java.util.UUID;

public record EventResponse(UUID id, String name, String location, Instant startsAt) {
}
