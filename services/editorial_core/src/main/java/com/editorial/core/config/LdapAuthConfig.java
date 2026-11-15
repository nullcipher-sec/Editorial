package com.editorial.core.config;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.ldap.core.LdapTemplate;
import org.springframework.ldap.core.support.LdapContextSource;

@Configuration
@ConditionalOnProperty(name = "editorial.ldap.enabled", havingValue = "true")
public class LdapAuthConfig {

    @Bean
    LdapContextSource ldapContextSource(
            @Value("${editorial.ldap.url}") String url,
            @Value("${editorial.ldap.base:dc=editorial,dc=local}") String base,
            @Value("${editorial.ldap.manager-dn:}") String managerDn,
            @Value("${editorial.ldap.manager-password:}") String managerPassword
    ) {
        LdapContextSource source = new LdapContextSource();
        source.setUrl(url);
        source.setBase(base);
        if (managerDn != null && !managerDn.isBlank()) {
            source.setUserDn(managerDn);
            source.setPassword(managerPassword);
        }
        return source;
    }

    @Bean
    LdapTemplate ldapTemplate(LdapContextSource ldapContextSource) {
        return new LdapTemplate(ldapContextSource);
    }
}