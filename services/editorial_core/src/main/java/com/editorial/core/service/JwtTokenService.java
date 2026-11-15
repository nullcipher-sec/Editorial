package com.editorial.core.service;

import com.editorial.core.security.AuthenticatedUser;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.security.oauth2.jose.jws.MacAlgorithm;
import org.springframework.security.oauth2.jwt.JwsHeader;
import org.springframework.security.oauth2.jwt.JwtClaimsSet;
import org.springframework.security.oauth2.jwt.JwtEncoder;
import org.springframework.security.oauth2.jwt.JwtEncoderParameters;
import org.springframework.stereotype.Service;

import java.time.Instant;

@Service
@RequiredArgsConstructor
public class JwtTokenService {

    private final JwtEncoder jwtEncoder;

    @Value("${editorial.jwt.issuer:editorial-core}")
    private String issuer;

    @Value("${editorial.jwt.expiration-hours:8}")
    private long expirationHours;

    public String issueToken(AuthenticatedUser user) {
        Instant now = Instant.now();
        JwtClaimsSet claims = JwtClaimsSet.builder()
                .issuer(issuer)
                .subject(user.ldapUid())
                .issuedAt(now)
                .expiresAt(now.plusSeconds(expirationHours * 3600))
                .claim("area", user.area().name())
                .claim("uid", user.id().toString())
                .build();
        JwsHeader header = JwsHeader.with(MacAlgorithm.HS256).build();
        return jwtEncoder.encode(JwtEncoderParameters.from(header, claims)).getTokenValue();
    }
}