package com.editorial.core.model.response;

import java.util.UUID;

public record ClientResponse(UUID id, String fullName, String email, String phone) {
}
