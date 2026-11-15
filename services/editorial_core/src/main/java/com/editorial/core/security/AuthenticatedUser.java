package com.editorial.core.security;

import com.editorial.core.entity.AppUser;
import com.editorial.core.entity.AreaCode;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;

import java.util.Collection;
import java.util.List;
import java.util.UUID;

public record AuthenticatedUser(
        UUID id,
        String ldapUid,
        String displayName,
        AreaCode area,
        Collection<? extends GrantedAuthority> authorities
) {
    public static AuthenticatedUser from(AppUser user) {
        return new AuthenticatedUser(
                user.getId(),
                user.getLdapUid(),
                user.getDisplayName(),
                user.getArea(),
                List.of(new SimpleGrantedAuthority(AreaRole.roleName(user.getArea())))
        );
    }
}