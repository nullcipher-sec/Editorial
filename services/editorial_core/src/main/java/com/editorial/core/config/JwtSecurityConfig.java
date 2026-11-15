package com.editorial.core.config;

import com.editorial.core.security.AreaRole;
import com.nimbusds.jose.JWSAlgorithm;
import com.nimbusds.jose.jwk.JWKSet;
import com.nimbusds.jose.jwk.OctetSequenceKey;
import com.nimbusds.jose.jwk.source.ImmutableJWKSet;
import com.nimbusds.jose.jwk.source.JWKSource;
import com.nimbusds.jose.proc.SecurityContext;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.convert.converter.Converter;
import org.springframework.security.authentication.AbstractAuthenticationToken;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.oauth2.jwt.Jwt;
import org.springframework.security.oauth2.jwt.JwtDecoder;
import org.springframework.security.oauth2.jwt.JwtEncoder;
import org.springframework.security.oauth2.jwt.NimbusJwtDecoder;
import org.springframework.security.oauth2.jwt.NimbusJwtEncoder;
import org.springframework.security.oauth2.server.resource.authentication.JwtAuthenticationConverter;

import javax.crypto.SecretKey;
import javax.crypto.spec.SecretKeySpec;
import java.nio.charset.StandardCharsets;
import java.util.List;

@Configuration
public class JwtSecurityConfig {

    @Bean
    JwtDecoder jwtDecoder(@Value("${editorial.jwt.secret}") String secret) {
        return NimbusJwtDecoder.withSecretKey(signingKey(secret)).build();
    }

    @Bean
    JwtEncoder jwtEncoder(@Value("${editorial.jwt.secret}") String secret) {
        SecretKey key = signingKey(secret);
        OctetSequenceKey jwk = new OctetSequenceKey.Builder(key.getEncoded())
                .algorithm(JWSAlgorithm.HS256)
                .keyID("editorial-core-hmac")
                .build();
        JWKSource<SecurityContext> source = new ImmutableJWKSet<>(new JWKSet(jwk));
        return new NimbusJwtEncoder(source);
    }

    @Bean
    Converter<Jwt, AbstractAuthenticationToken> jwtAuthenticationConverter() {
        JwtAuthenticationConverter converter = new JwtAuthenticationConverter();
        converter.setJwtGrantedAuthoritiesConverter(jwt -> {
            String area = jwt.getClaimAsString("area");
            if (area == null || area.isBlank()) {
                return List.of();
            }
            return List.of(new SimpleGrantedAuthority(AreaRole.PREFIX + area.toUpperCase()));
        });
        return converter;
    }

    private static SecretKey signingKey(String secret) {
        byte[] bytes = secret.getBytes(StandardCharsets.UTF_8);
        if (bytes.length < 32) {
            throw new IllegalStateException("editorial.jwt.secret must be at least 32 characters");
        }
        return new SecretKeySpec(bytes, "HmacSHA256");
    }
}