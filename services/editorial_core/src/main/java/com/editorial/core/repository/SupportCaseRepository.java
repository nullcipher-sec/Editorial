package com.editorial.core.repository;

import com.editorial.core.entity.SupportCase;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.UUID;

public interface SupportCaseRepository extends JpaRepository<SupportCase, UUID> {
}
