package com.editorial.core.controller.internal;

import com.editorial.core.entity.Event;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.response.EventResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.EventRepository;
import com.editorial.core.util.Phase1Guard;
import io.swagger.v3.oas.annotations.tags.Tag;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/internal/events")
@RequiredArgsConstructor
@Tag(name = "Internal — Events")
public class EventController {

    private final EventRepository eventRepository;

    @GetMapping
    public PageResponse<EventResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = eventRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public void create(@RequestBody Object body) throws ControllerException {
        Phase1Guard.rejectMutation();
    }

    private EventResponse toResponse(Event event) {
        return new EventResponse(event.getId(), event.getName(), event.getLocation(), event.getStartsAt());
    }
}
