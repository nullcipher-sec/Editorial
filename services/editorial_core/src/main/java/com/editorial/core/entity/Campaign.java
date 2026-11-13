package com.editorial.core.entity;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.Getter;
import lombok.Setter;

import java.time.Instant;
import java.time.LocalDate;
import java.util.UUID;

@Entity
@Table(name = "campaign")
@Getter
@Setter
public class Campaign {

    @Id
    private UUID id;

    @Column(nullable = false)
    private String name;

    private String channel;

    @Column(name = "budget_cents")
    private Long budgetCents;

    @Column(name = "starts_on")
    private LocalDate startsOn;

    @Column(name = "ends_on")
    private LocalDate endsOn;

    @Column(name = "created_at", nullable = false)
    private Instant createdAt;
}
