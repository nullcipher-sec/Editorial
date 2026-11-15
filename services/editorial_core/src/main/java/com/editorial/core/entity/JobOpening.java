package com.editorial.core.entity;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.EnumType;
import jakarta.persistence.Enumerated;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.Getter;
import lombok.Setter;

import java.time.Instant;
import java.util.UUID;

@Entity
@Table(name = "job_opening")
@Getter
@Setter
public class JobOpening {

    @Id
    private UUID id;

    @Column(nullable = false)
    private String title;

    private String description;

    @Enumerated(EnumType.STRING)
    @Column(nullable = false, length = 32)
    private AreaCode area;

    @Column(name = "open", nullable = false)
    private boolean open;

    @Column(name = "created_at", nullable = false)
    private Instant createdAt;
}
