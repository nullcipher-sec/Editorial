package com.editorial.core.controller.internal;

import com.editorial.core.entity.Candidate;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.request.CreateCandidateRequest;
import com.editorial.core.model.response.CandidateResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.CandidateRepository;
import com.editorial.core.service.RecruitmentService;
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
@RequestMapping("/api/internal/recruitment/candidates")
@RequiredArgsConstructor
@PreAuthorize("hasAuthority('ROLE_RECRUITMENT')")
@Tag(name = "Internal — Recruitment")
public class RecruitmentController {

    private final CandidateRepository candidateRepository;
    private final RecruitmentService recruitmentService;

    @GetMapping
    public PageResponse<CandidateResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = candidateRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public CandidateResponse create(@Valid @RequestBody CreateCandidateRequest body) throws ControllerException {
        return recruitmentService.createCandidate(body);
    }

    private CandidateResponse toResponse(Candidate candidate) {
        return new CandidateResponse(
                candidate.getId(),
                candidate.getJobOpening().getId(),
                candidate.getFullName(),
                candidate.getStage()
        );
    }
}