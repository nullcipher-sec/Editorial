package com.editorial.core.controller.internal;

import com.editorial.core.entity.Campaign;
import com.editorial.core.model.request.CreateCampaignRequest;
import com.editorial.core.model.response.CampaignResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.CampaignRepository;
import com.editorial.core.service.MarketingService;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/internal/marketing/campaigns")
@RequiredArgsConstructor
@PreAuthorize("hasAuthority('ROLE_MARKETING')")
@Tag(name = "Internal — Marketing campaigns")
public class MarketingController {

    private final CampaignRepository campaignRepository;
    private final MarketingService marketingService;

    @GetMapping
    public PageResponse<CampaignResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = campaignRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public CampaignResponse create(@Valid @RequestBody CreateCampaignRequest body) {
        return marketingService.create(body);
    }

    private CampaignResponse toResponse(Campaign campaign) {
        return new CampaignResponse(campaign.getId(), campaign.getName(), campaign.getChannel());
    }
}