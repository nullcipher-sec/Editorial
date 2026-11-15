package com.editorial.core.controller.internal;

import com.editorial.core.entity.SupportCase;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.request.CreateCaseRequest;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.model.response.SupportCaseResponse;
import com.editorial.core.repository.SupportCaseRepository;
import com.editorial.core.service.AttentionService;
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
@RequestMapping("/api/internal/attention/cases")
@RequiredArgsConstructor
@PreAuthorize("hasAuthority('ROLE_ATTENTION')")
@Tag(name = "Internal — Attention cases")
public class AttentionCaseController {

    private final SupportCaseRepository supportCaseRepository;
    private final AttentionService attentionService;

    @GetMapping
    public PageResponse<SupportCaseResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = supportCaseRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public SupportCaseResponse create(@Valid @RequestBody CreateCaseRequest body) throws ControllerException {
        return attentionService.createCase(body);
    }

    private SupportCaseResponse toResponse(SupportCase supportCase) {
        return AttentionService.toCaseResponse(supportCase);
    }
}