package com.editorial.core.model.response;

import java.util.UUID;

public record JobOpeningResponse(UUID id, String title, String area, boolean open) {
}
