package com.editorial.core.service;

import com.editorial.core.entity.Campaign;
import com.editorial.core.model.request.CreateCampaignRequest;
import com.editorial.core.model.response.CampaignResponse;
import com.editorial.core.repository.CampaignRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.Instant;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class MarketingService {

    private final CampaignRepository campaignRepository;

    @Transactional
    public CampaignResponse create(CreateCampaignRequest request) {
        Campaign campaign = new Campaign();
        campaign.setId(UUID.randomUUID());
        campaign.setName(request.getName().trim());
        campaign.setChannel(request.getChannel());
        campaign.setCreatedAt(Instant.now());
        campaign = campaignRepository.save(campaign);
        return new CampaignResponse(campaign.getId(), campaign.getName(), campaign.getChannel());
    }
}