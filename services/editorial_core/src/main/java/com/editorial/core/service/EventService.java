package com.editorial.core.service;

import com.editorial.core.entity.Event;
import com.editorial.core.model.request.CreateEventRequest;
import com.editorial.core.model.response.EventResponse;
import com.editorial.core.repository.EventRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.Instant;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class EventService {

    private final EventRepository eventRepository;

    @Transactional
    public EventResponse create(CreateEventRequest request) {
        Event event = new Event();
        event.setId(UUID.randomUUID());
        event.setName(request.getName().trim());
        event.setDescription(request.getDescription());
        event.setLocation(request.getLocation());
        event.setStartsAt(request.getStartsAt());
        event.setEndsAt(request.getEndsAt());
        event.setCreatedAt(Instant.now());
        event = eventRepository.save(event);
        return new EventResponse(event.getId(), event.getName(), event.getLocation(), event.getStartsAt());
    }
}