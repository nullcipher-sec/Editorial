package com.editorial.core.model.response;

import java.util.UUID;

public record SupportCaseResponse(UUID id, UUID clientId, String subject, String status) {
}
