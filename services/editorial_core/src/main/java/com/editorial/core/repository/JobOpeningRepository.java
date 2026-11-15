package com.editorial.core.repository;

import com.editorial.core.entity.JobOpening;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.UUID;

public interface JobOpeningRepository extends JpaRepository<JobOpening, UUID> {
}
