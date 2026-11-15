package com.editorial.core.model.response;

import com.editorial.core.entity.AreaCode;
import com.editorial.core.security.AuthenticatedUser;
import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.UUID;

public record SessionResponse(SessionUser user, SessionToken token) {

    public static SessionResponse of(AuthenticatedUser authUser, String jwt) {
        return new SessionResponse(
                new SessionUser(authUser.id(), authUser.ldapUid(), authUser.displayName(), authUser.area()),
                new SessionToken(jwt)
        );
    }

    public record SessionUser(
            UUID id,
            @JsonProperty("ldap_uid") String ldapUid,
            @JsonProperty("display_name") String displayName,
            AreaCode area
    ) {
    }

    public record SessionToken(String value) {
    }
}