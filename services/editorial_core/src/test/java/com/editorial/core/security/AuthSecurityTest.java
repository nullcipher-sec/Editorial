package com.editorial.core.security;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.MediaType;
import org.springframework.test.context.ActiveProfiles;
import org.springframework.test.context.TestPropertySource;
import org.springframework.test.context.jdbc.Sql;
import org.springframework.test.web.servlet.MockMvc;

import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.post;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

@SpringBootTest
@AutoConfigureMockMvc
@ActiveProfiles("dev")
@TestPropertySource(properties = {
        "spring.datasource.url=jdbc:h2:mem:editorial-auth;MODE=PostgreSQL;DB_CLOSE_DELAY=-1",
        "spring.datasource.username=sa",
        "spring.datasource.password=",
        "spring.datasource.driver-class-name=org.h2.Driver",
        "spring.jpa.hibernate.ddl-auto=create-drop",
        "POSTGRES_USER=sa",
        "POSTGRES_PASSWORD=",
        "editorial.jwt.secret=local-dev-only-secret-32-chars-min!!",
        "editorial.ldap.enabled=false",
        "editorial.auth.dev-password=secret"
})
@Sql(scripts = "/data-auth.sql", executionPhase = Sql.ExecutionPhase.BEFORE_TEST_METHOD)
class AuthSecurityTest {

    @Autowired
    private MockMvc mockMvc;

    @Test
    void loginAndAreaAuthorization() throws Exception {
        mockMvc.perform(get("/api/internal/attention/clients"))
                .andExpect(status().isUnauthorized());

        String loginBody = """
                {"username":"atencion.agente","password":"secret"}
                """;
        String token = mockMvc.perform(post("/api/auth/login")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content(loginBody))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.token.value").isNotEmpty())
                .andReturn()
                .getResponse()
                .getContentAsString();

        // Extrae token sin depender de Jackson en el test
        int start = token.indexOf("\"value\":\"") + 9;
        int end = token.indexOf('"', start);
        String jwt = token.substring(start, end);

        mockMvc.perform(get("/api/internal/attention/clients")
                        .header("Authorization", "Bearer " + jwt))
                .andExpect(status().isOk());

        mockMvc.perform(get("/api/internal/content/articles")
                        .header("Authorization", "Bearer " + jwt))
                .andExpect(status().isForbidden());
    }
}