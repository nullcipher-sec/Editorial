package com.editorial.core.model.response;

import java.util.UUID;

public record CampaignResponse(UUID id, String name, String channel) {
}
