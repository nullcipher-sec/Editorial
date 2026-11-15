package com.editorial.core.service;

import com.editorial.core.entity.Candidate;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;
import com.editorial.core.model.request.CreateCandidateRequest;
import com.editorial.core.model.response.CandidateResponse;
import com.editorial.core.repository.CandidateRepository;
import com.editorial.core.repository.JobOpeningRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.Instant;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class RecruitmentService {

    private final CandidateRepository candidateRepository;
    private final JobOpeningRepository jobOpeningRepository;

    @Transactional
    public CandidateResponse createCandidate(CreateCandidateRequest request) throws ControllerException {
        var job = jobOpeningRepository.findById(request.getJobOpeningId())
                .orElseThrow(() -> new ControllerException(ErrorResponses.NOT_FOUND));
        Candidate candidate = new Candidate();
        candidate.setId(UUID.randomUUID());
        candidate.setJobOpening(job);
        candidate.setFullName(request.getFullName().trim());
        candidate.setEmail(request.getEmail().trim());
        candidate.setResumeSummary(request.getResumeSummary());
        candidate.setStage("APPLIED");
        candidate.setCreatedAt(Instant.now());
        candidate = candidateRepository.save(candidate);
        return new CandidateResponse(
                candidate.getId(),
                job.getId(),
                candidate.getFullName(),
                candidate.getStage()
        );
    }
}