package com.editorial.core.security;

import com.editorial.core.entity.AppUser;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;
import com.editorial.core.repository.AppUserRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.oauth2.jwt.Jwt;
import org.springframework.stereotype.Service;

import java.util.UUID;

@Service
@RequiredArgsConstructor
public class SecurityPrincipalService {

    private final AppUserRepository appUserRepository;

    public AppUser requireCurrentUser() throws ControllerException {
        Authentication authentication = SecurityContextHolder.getContext().getAuthentication();
        if (authentication == null || !(authentication.getPrincipal() instanceof Jwt jwt)) {
            throw new ControllerException(ErrorResponses.UNAUTHORIZED);
        }
        String uid = jwt.getClaimAsString("uid");
        if (uid == null) {
            String subject = jwt.getSubject();
            return appUserRepository.findByLdapUidIgnoreCase(subject)
                    .orElseThrow(() -> new ControllerException(ErrorResponses.UNAUTHORIZED));
        }
        return appUserRepository.findById(UUID.fromString(uid))
                .orElseThrow(() -> new ControllerException(ErrorResponses.UNAUTHORIZED));
    }
}