package com.editorial.core.controller.internal;

import com.editorial.core.entity.Article;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.response.ArticleSummaryResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.ArticleRepository;
import com.editorial.core.util.Phase1Guard;
import io.swagger.v3.oas.annotations.tags.Tag;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
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
@Tag(name = "Internal — Content articles")
public class ContentArticleController {

    private final ArticleRepository articleRepository;

    @GetMapping
    public PageResponse<ArticleSummaryResponse> list(@PageableDefault(size = 20) Pageable pageable) {
        var page = articleRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toSummary).getContent());
    }

    @PostMapping
    public void create(@RequestBody Object body) throws ControllerException {
        Phase1Guard.rejectMutation();
    }

    @PatchMapping("/{id}/publish")
    public void publish(@PathVariable UUID id) throws ControllerException {
        Phase1Guard.rejectMutation();
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
