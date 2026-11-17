package com.editorial.core.repository;

import com.editorial.core.entity.SupportCase;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.repository.EntityGraph;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.UUID;

public interface SupportCaseRepository extends JpaRepository<SupportCase, UUID> {
    @EntityGraph(attributePaths = {"client"})
    Page<SupportCase> findAll(Pageable pageable);

    void deleteAllByClientId(UUID clientId);
}
