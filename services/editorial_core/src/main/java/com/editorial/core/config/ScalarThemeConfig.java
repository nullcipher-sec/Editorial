package com.editorial.core.config;

import com.scalar.maven.webmvc.SpringBootScalarProperties;
import org.springframework.beans.factory.config.BeanPostProcessor;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.io.ClassPathResource;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Base64;

/**
 * Identidad visual de Sultana del Lago Editores en Scalar (solo perfil {@code dev}).
 */
@Configuration
@ConditionalOnProperty(name = "scalar.enabled", havingValue = "true")
public class ScalarThemeConfig {

    private static final String THEME_CSS = "scalar/sultana-theme.css";
    private static final String MARK_SVG = "scalar/sultana-mark.svg";

    @Bean
    static BeanPostProcessor scalarThemeApplier() throws IOException {
        String css = read(THEME_CSS);
        String favicon = "data:image/svg+xml;base64,"
                + Base64.getEncoder().encodeToString(read(MARK_SVG).getBytes(StandardCharsets.UTF_8));
        return new BeanPostProcessor() {
            @Override
            public Object postProcessAfterInitialization(Object bean, String beanName) {
                if (bean instanceof SpringBootScalarProperties properties) {
                    properties.setCustomCss(css);
                    properties.setFavicon(favicon);
                }
                return bean;
            }
        };
    }

    private static String read(String location) throws IOException {
        return new ClassPathResource(location).getContentAsString(StandardCharsets.UTF_8);
    }
}
