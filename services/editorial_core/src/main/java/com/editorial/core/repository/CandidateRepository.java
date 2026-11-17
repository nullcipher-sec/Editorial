package com.editorial.core.repository;

import com.editorial.core.entity.Candidate;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.repository.EntityGraph;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.UUID;

public interface CandidateRepository extends JpaRepository<Candidate, UUID> {
    @EntityGraph(attributePaths = {"jobOpening"})
    Page<Candidate> findAll(Pageable pageable);
}
