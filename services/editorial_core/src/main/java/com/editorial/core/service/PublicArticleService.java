package com.editorial.core.service;

import com.editorial.core.entity.Article;
import com.editorial.core.entity.ArticleStatus;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;
import com.editorial.core.model.response.PublicArticleResponse;
import com.editorial.core.repository.ArticleRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.UUID;

@Service
@RequiredArgsConstructor
public class PublicArticleService {

    private final ArticleRepository articleRepository;

    @Transactional(readOnly = true)
    public Page<PublicArticleResponse> listPublished(Pageable pageable) {
        return articleRepository
                .findByStatusOrderByPublishedAtDesc(ArticleStatus.PUBLISHED, pageable)
                .map(this::toResponse);
    }

    @Transactional(readOnly = true)
    public PublicArticleResponse getPublishedById(UUID id) throws ControllerException {
        Article article = articleRepository.findByIdAndStatus(id, ArticleStatus.PUBLISHED)
                .orElseThrow(() -> new ControllerException(ErrorResponses.NOT_FOUND));
        return toResponse(article);
    }

    private PublicArticleResponse toResponse(Article article) {
        return new PublicArticleResponse(
                article.getId(),
                article.getTitle(),
                article.getSummary(),
                article.getBody(),
                article.getPublishedAt()
        );
    }
}
