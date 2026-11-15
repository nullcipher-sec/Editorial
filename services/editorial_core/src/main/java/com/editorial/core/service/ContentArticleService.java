package com.editorial.core.service;

import com.editorial.core.entity.Article;
import com.editorial.core.entity.ArticleStatus;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;
import com.editorial.core.model.request.CreateArticleRequest;
import com.editorial.core.model.response.ArticleResponse;
import com.editorial.core.model.response.ArticleSummaryResponse;
import com.editorial.core.repository.ArticleRepository;
import com.editorial.core.security.SecurityPrincipalService;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.Instant;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class ContentArticleService {

    private final ArticleRepository articleRepository;
    private final SecurityPrincipalService securityPrincipalService;

    @Transactional
    public ArticleResponse create(CreateArticleRequest request) throws ControllerException {
        var author = securityPrincipalService.requireCurrentUser();
        Instant now = Instant.now();
        Article article = new Article();
        article.setId(UUID.randomUUID());
        article.setTitle(request.getTitle().trim());
        article.setSummary(request.getSummary());
        article.setBody(request.getBody().trim());
        article.setStatus(ArticleStatus.DRAFT);
        article.setAuthor(author);
        article.setCreatedAt(now);
        article.setUpdatedAt(now);
        return toResponse(articleRepository.save(article));
    }

    @Transactional
    public ArticleSummaryResponse publish(UUID id) throws ControllerException {
        Article article = articleRepository.findById(id)
                .orElseThrow(() -> new ControllerException(ErrorResponses.NOT_FOUND));
        Instant now = Instant.now();
        article.setStatus(ArticleStatus.PUBLISHED);
        article.setPublishedAt(now);
        article.setUpdatedAt(now);
        articleRepository.save(article);
        return new ArticleSummaryResponse(
                article.getId(),
                article.getTitle(),
                article.getStatus().name(),
                article.getPublishedAt()
        );
    }

    static ArticleResponse toResponse(Article article) {
        return new ArticleResponse(
                article.getId(),
                article.getTitle(),
                article.getSummary(),
                article.getBody(),
                article.getStatus(),
                article.getAuthor() != null ? article.getAuthor().getId() : null,
                article.getPublishedAt()
        );
    }
}