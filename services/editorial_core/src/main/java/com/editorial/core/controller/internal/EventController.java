package com.editorial.core.controller.internal;

import com.editorial.core.entity.Event;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.request.CreateEventRequest;
import com.editorial.core.model.response.EventResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.EventRepository;
import com.editorial.core.service.EventService;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.UUID;

@RestController
@RequestMapping("/api/internal/events")
@RequiredArgsConstructor
@PreAuthorize("hasAuthority('ROLE_MARKETING')")
@Tag(name = "Internal — Events")
public class EventController {

    private final EventRepository eventRepository;
    private final EventService eventService;

    @GetMapping
    public PageResponse<EventResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = eventRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public EventResponse create(@Valid @RequestBody CreateEventRequest body) {
        return eventService.create(body);
    }

    @DeleteMapping("/{id}")
    public void delete(@PathVariable UUID id) throws ControllerException {
        eventService.delete(id);
    }

    private EventResponse toResponse(Event event) {
        return new EventResponse(event.getId(), event.getName(), event.getLocation(), event.getStartsAt());
    }
}