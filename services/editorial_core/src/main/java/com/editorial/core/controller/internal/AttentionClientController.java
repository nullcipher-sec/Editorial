package com.editorial.core.controller.internal;

import com.editorial.core.entity.Client;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.request.CreateClientRequest;
import com.editorial.core.model.response.ClientResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.ClientRepository;
import com.editorial.core.service.AttentionService;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/internal/attention/clients")
@RequiredArgsConstructor
@PreAuthorize("hasAuthority('ROLE_ATTENTION')")
@Tag(name = "Internal — Attention clients")
public class AttentionClientController {

    private final ClientRepository clientRepository;
    private final AttentionService attentionService;

    @GetMapping
    public PageResponse<ClientResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = clientRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toResponse).getContent());
    }

    @PostMapping
    public ClientResponse create(@Valid @RequestBody CreateClientRequest body) {
        return attentionService.createClient(body);
    }

    private ClientResponse toResponse(Client client) {
        return AttentionService.toClientResponse(client);
    }
}