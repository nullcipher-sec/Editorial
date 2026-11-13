package com.editorial.core.config;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.Contact;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.servers.Server;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Profile;

import java.util.List;

@Configuration
@Profile("dev")
public class OpenApiConfig {

    @Bean
    OpenAPI editorialOpenApi(@Value("${server.port:8080}") int port) {
        return new OpenAPI()
                .info(new Info()
                        .title("Sultana del Lago Editores — Core API")
                        .description("Contrato fase 1: rutas estables; mutaciones internas pendientes de auth LDAP/JWT.")
                        .version("0.1.0")
                        .contact(new Contact().name("@marcelopcx").url("https://github.com/nullcipher-sec/Editorial")))
                .servers(List.of(
                        new Server().url("http://localhost:" + port).description("Desarrollo local"),
                        new Server().url("http://10.20.30.10:8080").description("SRV-Apps-E (VLAN 30)")));
    }
}
