package com.editorial.core.controller.internal;

import com.editorial.core.entity.SupportCase;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.model.response.SupportCaseResponse;
import com.editorial.core.repository.SupportCaseRepository;
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
@RequestMapping("/api/internal/attention/cases")
@RequiredArgsConstructor
@Tag(name = "Internal — Attention cases")
public class AttentionCaseController {

    private final SupportCaseRepository supportCaseRepository;

    @GetMapping
    public PageResponse<SupportCaseResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = supportCaseRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public void create(@RequestBody Object body) throws ControllerException {
        Phase1Guard.rejectMutation();
    }

    private SupportCaseResponse toResponse(SupportCase supportCase) {
        return new SupportCaseResponse(
                supportCase.getId(),
                supportCase.getClient().getId(),
                supportCase.getSubject(),
                supportCase.getStatus().name()
        );
    }
}
