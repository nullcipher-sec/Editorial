package com.editorial.core.controller;

import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.model.response.PublicArticleResponse;
import com.editorial.core.service.PublicArticleService;
import io.swagger.v3.oas.annotations.tags.Tag;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.UUID;

@RestController
@RequestMapping("/api/public/articles")
@RequiredArgsConstructor
@Tag(name = "Public — Articles", description = "Noticiero web (@JhonmarMorales17): solo artículos publicados")
public class PublicArticleController {

    private final PublicArticleService publicArticleService;

    @GetMapping
    public PageResponse<PublicArticleResponse> list(
            @PageableDefault(size = 20) Pageable pageable) {
        var page = publicArticleService.listPublished(pageable);
        return PageResponse.from(page, page.getContent());
    }

    @GetMapping("/{id}")
    public PublicArticleResponse getById(@PathVariable UUID id) throws ControllerException {
        return publicArticleService.getPublishedById(id);
    }
}
