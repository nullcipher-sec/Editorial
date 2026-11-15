package com.editorial.core.service;

import com.editorial.core.entity.AppUser;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;
import com.editorial.core.repository.AppUserRepository;
import com.editorial.core.security.AuthenticatedUser;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.ObjectProvider;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.ldap.core.LdapTemplate;
import org.springframework.ldap.filter.EqualsFilter;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class AuthenticationService {

    private final AppUserRepository appUserRepository;
    private final JwtTokenService jwtTokenService;
    private final ObjectProvider<LdapTemplate> ldapTemplate;

    @Value("${editorial.ldap.enabled:false}")
    private boolean ldapEnabled;

    @Value("${editorial.auth.dev-password:}")
    private String devPassword;

    public LoginResult login(String username, String password) throws ControllerException {
        AppUser user = appUserRepository.findByLdapUidIgnoreCase(username.trim())
                .orElseThrow(() -> new ControllerException(ErrorResponses.INVALID_CREDENTIALS));

        if (!verifyPassword(username.trim(), password)) {
            throw new ControllerException(ErrorResponses.INVALID_CREDENTIALS);
        }

        AuthenticatedUser authUser = AuthenticatedUser.from(user);
        String token = jwtTokenService.issueToken(authUser);
        return new LoginResult(authUser, token);
    }

    private boolean verifyPassword(String username, String rawPassword) {
        if (ldapEnabled) {
            LdapTemplate ldap = ldapTemplate.getIfAvailable();
            if (ldap == null) {
                return false;
            }
            EqualsFilter filter = new EqualsFilter("uid", username);
            return ldap.authenticate("ou=people", filter.encode(), rawPassword);
        }
        if (devPassword == null || devPassword.isBlank()) {
            return false;
        }
        return constantTimeEquals(rawPassword, devPassword);
    }

    private static boolean constantTimeEquals(String a, String b) {
        if (a == null || b == null || a.length() != b.length()) {
            return false;
        }
        int result = 0;
        for (int i = 0; i < a.length(); i++) {
            result |= a.charAt(i) ^ b.charAt(i);
        }
        return result == 0;
    }

    public record LoginResult(AuthenticatedUser user, String token) {
    }
}