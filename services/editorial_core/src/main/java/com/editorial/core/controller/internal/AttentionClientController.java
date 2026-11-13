package com.editorial.core.controller.internal;

import com.editorial.core.entity.Client;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.model.response.ClientResponse;
import com.editorial.core.repository.ClientRepository;
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
@RequestMapping("/api/internal/attention/clients")
@RequiredArgsConstructor
@Tag(name = "Internal — Attention clients")
public class AttentionClientController {

    private final ClientRepository clientRepository;

    @GetMapping
    public PageResponse<ClientResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = clientRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public void create(@RequestBody Object body) throws ControllerException {
        Phase1Guard.rejectMutation();
    }

    private ClientResponse toResponse(Client client) {
        return new ClientResponse(client.getId(), client.getFullName(), client.getEmail(), client.getPhone());
    }
}
