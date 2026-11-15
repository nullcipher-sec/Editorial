package com.editorial.core.controller;

import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.request.LoginRequest;
import com.editorial.core.model.response.SessionResponse;
import com.editorial.core.service.AuthenticationService;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/auth")
@RequiredArgsConstructor
@Tag(name = "Auth")
public class AuthController {

    private final AuthenticationService authenticationService;

    @PostMapping("/login")
    public SessionResponse login(@Valid @RequestBody LoginRequest request) throws ControllerException {
        var result = authenticationService.login(request.getUsername(), request.getPassword());
        return SessionResponse.of(result.user(), result.token());
    }
}