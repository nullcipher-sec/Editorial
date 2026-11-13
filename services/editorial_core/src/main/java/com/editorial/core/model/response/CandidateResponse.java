package com.editorial.core.model.response;

import java.util.UUID;

public record CandidateResponse(UUID id, UUID jobOpeningId, String fullName, String stage) {
}
