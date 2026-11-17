package com.editorial.core.controller.internal;

import com.editorial.core.entity.Article;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.request.CreateArticleRequest;
import com.editorial.core.model.response.ArticleResponse;
import com.editorial.core.model.response.ArticleSummaryResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.ArticleRepository;
import com.editorial.core.service.ContentArticleService;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PatchMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.UUID;

@RestController
@RequestMapping("/api/internal/content/articles")
@RequiredArgsConstructor
@PreAuthorize("hasAuthority('ROLE_PRODUCTION')")
@Tag(name = "Internal — Content articles")
public class ContentArticleController {

    private final ArticleRepository articleRepository;
    private final ContentArticleService contentArticleService;

    @GetMapping
    public PageResponse<ArticleSummaryResponse> list(@PageableDefault(size = 20, sort = {"createdAt", "id"}) Pageable pageable) {
        var page = articleRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toSummary).getContent());
    }

    @PostMapping
    public ArticleResponse create(@Valid @RequestBody CreateArticleRequest body) throws ControllerException {
        return contentArticleService.create(body);
    }

    @GetMapping("/{id}")
    public ArticleResponse get(@PathVariable UUID id) throws ControllerException {
        return contentArticleService.get(id);
    }

    @PatchMapping("/{id}/review")
    public ArticleResponse sendToReview(@PathVariable UUID id) throws ControllerException {
        return contentArticleService.sendToReview(id);
    }

    @PatchMapping("/{id}/publish")
    public ArticleResponse publish(@PathVariable UUID id) throws ControllerException {
        return contentArticleService.publish(id);
    }

    @DeleteMapping("/{id}")
    public void delete(@PathVariable UUID id) throws ControllerException {
        contentArticleService.delete(id);
    }

    private ArticleSummaryResponse toSummary(Article article) {
        return new ArticleSummaryResponse(
                article.getId(),
                article.getTitle(),
                article.getStatus().name(),
                article.getPublishedAt()
        );
    }
}