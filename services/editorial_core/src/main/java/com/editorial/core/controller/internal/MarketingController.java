package com.editorial.core.controller.internal;

import com.editorial.core.entity.Campaign;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.response.CampaignResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.CampaignRepository;
import com.editorial.core.util.Phase1Guard;
import io.swagger.v3.oas.annotations.tags.Tag;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/internal/marketing/campaigns")
@RequiredArgsConstructor
@Tag(name = "Internal — Marketing campaigns")
public class MarketingController {

    private final CampaignRepository campaignRepository;

    @GetMapping
    public PageResponse<CampaignResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = campaignRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public void create(@RequestBody Object body) throws ControllerException {
        Phase1Guard.rejectMutation();
    }

    private CampaignResponse toResponse(Campaign campaign) {
        return new CampaignResponse(campaign.getId(), campaign.getName(), campaign.getChannel());
    }
}
