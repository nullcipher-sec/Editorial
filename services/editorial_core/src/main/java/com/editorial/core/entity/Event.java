package com.editorial.core.entity;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.Getter;
import lombok.Setter;

import java.time.Instant;
import java.util.UUID;

@Entity
@Table(name = "event")
@Getter
@Setter
public class Event {

    @Id
    private UUID id;

    @Column(nullable = false)
    private String name;

    private String description;

    private String location;

    @Column(name = "starts_at", nullable = false)
    private Instant startsAt;

    @Column(name = "ends_at")
    private Instant endsAt;

    @Column(name = "created_at", nullable = false)
    private Instant createdAt;
}
