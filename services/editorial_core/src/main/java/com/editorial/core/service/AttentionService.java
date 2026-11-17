package com.editorial.core.service;

import com.editorial.core.entity.CaseStatus;
import com.editorial.core.entity.Client;
import com.editorial.core.entity.SupportCase;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;
import com.editorial.core.model.request.CreateCaseRequest;
import com.editorial.core.model.request.CreateClientRequest;
import com.editorial.core.model.request.UpdateCaseStatusRequest;
import com.editorial.core.model.response.ClientResponse;
import com.editorial.core.model.response.SupportCaseResponse;
import com.editorial.core.repository.ClientRepository;
import com.editorial.core.repository.SupportCaseRepository;
import com.editorial.core.security.SecurityPrincipalService;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.Instant;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class AttentionService {

    private final ClientRepository clientRepository;
    private final SupportCaseRepository supportCaseRepository;
    private final SecurityPrincipalService securityPrincipalService;

    @Transactional
    public ClientResponse createClient(CreateClientRequest request) {
        Client client = new Client();
        client.setId(UUID.randomUUID());
        client.setFullName(request.getFullName().trim());
        String email = request.getEmail() != null && !request.getEmail().isBlank() ? request.getEmail().trim() : null;
        String phone = request.getPhone() != null && !request.getPhone().isBlank() ? request.getPhone().trim() : null;
        client.setEmail(email);
        client.setPhone(phone);
        client.setCreatedAt(Instant.now());
        return toClientResponse(clientRepository.save(client));
    }

    @Transactional
    public SupportCaseResponse createCase(CreateCaseRequest request) throws ControllerException {
        Client client = clientRepository.findById(request.getClientId())
                .orElseThrow(() -> new ControllerException(ErrorResponses.NOT_FOUND));
        var assignee = securityPrincipalService.requireCurrentUser();
        Instant now = Instant.now();
        SupportCase supportCase = new SupportCase();
        supportCase.setId(UUID.randomUUID());
        supportCase.setClient(client);
        supportCase.setSubject(request.getSubject().trim());
        supportCase.setDescription(request.getDescription().trim());
        supportCase.setStatus(CaseStatus.OPEN);
        supportCase.setAssignedTo(assignee);
        supportCase.setCreatedAt(now);
        supportCase.setUpdatedAt(now);
        return toCaseResponse(supportCaseRepository.save(supportCase));
    }

    @Transactional
    public SupportCaseResponse updateCaseStatus(UUID id, UpdateCaseStatusRequest request) throws ControllerException {
        SupportCase supportCase = supportCaseRepository.findById(id)
                .orElseThrow(() -> new ControllerException(ErrorResponses.NOT_FOUND));
        supportCase.setStatus(request.getStatus());
        supportCase.setUpdatedAt(Instant.now());
        return toCaseResponse(supportCaseRepository.save(supportCase));
    }

    @Transactional
    public void deleteCase(UUID id) throws ControllerException {
        SupportCase supportCase = supportCaseRepository.findById(id)
                .orElseThrow(() -> new ControllerException(ErrorResponses.NOT_FOUND));
        supportCaseRepository.delete(supportCase);
    }

    @Transactional
    public void deleteClient(UUID id) throws ControllerException {
        Client client = clientRepository.findById(id)
                .orElseThrow(() -> new ControllerException(ErrorResponses.NOT_FOUND));
        supportCaseRepository.deleteAllByClientId(id);
        clientRepository.delete(client);
    }

    public static ClientResponse toClientResponse(Client client) {
        return new ClientResponse(client.getId(), client.getFullName(), client.getEmail(), client.getPhone());
    }

    public static SupportCaseResponse toCaseResponse(SupportCase supportCase) {
        return new SupportCaseResponse(
                supportCase.getId(),
                supportCase.getClient().getId(),
                supportCase.getSubject(),
                supportCase.getStatus().name()
        );
    }
}