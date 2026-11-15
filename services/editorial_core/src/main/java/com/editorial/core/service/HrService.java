package com.editorial.core.service;

import com.editorial.core.entity.JobOpening;
import com.editorial.core.model.request.CreateJobOpeningRequest;
import com.editorial.core.model.response.JobOpeningResponse;
import com.editorial.core.repository.JobOpeningRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.Instant;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class HrService {

    private final JobOpeningRepository jobOpeningRepository;

    @Transactional
    public JobOpeningResponse createJobOpening(CreateJobOpeningRequest request) {
        JobOpening opening = new JobOpening();
        opening.setId(UUID.randomUUID());
        opening.setTitle(request.getTitle().trim());
        opening.setDescription(request.getDescription());
        opening.setArea(request.getArea());
        opening.setOpen(true);
        opening.setCreatedAt(Instant.now());
        opening = jobOpeningRepository.save(opening);
        return new JobOpeningResponse(
                opening.getId(),
                opening.getTitle(),
                opening.getArea().name(),
                opening.isOpen()
        );
    }
}