package com.editorial.core.service;

import com.editorial.core.entity.AppUser;
import com.editorial.core.entity.AreaCode;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;
import com.editorial.core.repository.AppUserRepository;
import com.editorial.core.security.AuthenticatedUser;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.ObjectProvider;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.ldap.core.LdapTemplate;
import org.springframework.ldap.filter.EqualsFilter;
import org.springframework.ldap.query.LdapQueryBuilder;
import org.springframework.stereotype.Service;

import java.time.Instant;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

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
        String uid = username.trim();
        AppUser user;
        if (ldapEnabled) {
            // LDAP es la fuente de verdad: valida la clave y aporta nombre y área.
            if (!verifyPassword(uid, password)) {
                throw new ControllerException(ErrorResponses.INVALID_CREDENTIALS);
            }
            LdapProfile profile = loadLdapProfile(uid)
                    .orElseThrow(() -> new ControllerException(ErrorResponses.FORBIDDEN));
            user = provisionUser(uid, profile);
        } else {
            user = appUserRepository.findByLdapUidIgnoreCase(uid)
                    .orElseThrow(() -> new ControllerException(ErrorResponses.INVALID_CREDENTIALS));
            if (!verifyPassword(uid, password)) {
                throw new ControllerException(ErrorResponses.INVALID_CREDENTIALS);
            }
        }

        AuthenticatedUser authUser = AuthenticatedUser.from(user);
        String token = jwtTokenService.issueToken(authUser);
        return new LoginResult(authUser, token);
    }

    /** Crea el usuario en app_user la primera vez, o sincroniza nombre y área con LDAP. */
    private AppUser provisionUser(String uid, LdapProfile profile) {
        AppUser user = appUserRepository.findByLdapUidIgnoreCase(uid).orElse(null);
        if (user == null) {
            user = new AppUser();
            user.setId(UUID.randomUUID());
            user.setLdapUid(uid);
            user.setCreatedAt(Instant.now());
        }
        user.setDisplayName(profile.displayName());
        user.setArea(profile.area());
        return appUserRepository.save(user);
    }

    /** Lee de LDAP el nombre (cn) y el área, según el grupo (gidNumber) del usuario. */
    private Optional<LdapProfile> loadLdapProfile(String uid) {
        LdapTemplate ldap = ldapTemplate.getIfAvailable();
        if (ldap == null) {
            return Optional.empty();
        }
        List<String[]> people = ldap.search(
                LdapQueryBuilder.query().base("ou=people").where("uid").is(uid),
                (javax.naming.directory.Attributes attrs) -> new String[]{
                        attrs.get("cn") != null ? String.valueOf(attrs.get("cn").get()) : uid,
                        attrs.get("gidNumber") != null ? String.valueOf(attrs.get("gidNumber").get()) : null
                });
        if (people.isEmpty() || people.get(0)[1] == null) {
            return Optional.empty();
        }
        String[] person = people.get(0);
        List<String> groups = ldap.search(
                LdapQueryBuilder.query().base("ou=groups").where("gidNumber").is(person[1]),
                (javax.naming.directory.Attributes attrs) -> String.valueOf(attrs.get("cn").get()));
        for (String group : groups) {
            try {
                return Optional.of(new LdapProfile(person[0], AreaCode.valueOf(group.toUpperCase())));
            } catch (IllegalArgumentException ignored) {
                // grupo que no corresponde a un área del sistema
            }
        }
        return Optional.empty();
    }

    private record LdapProfile(String displayName, AreaCode area) {
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