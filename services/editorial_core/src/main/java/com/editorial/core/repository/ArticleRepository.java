package com.editorial.core.repository;

import com.editorial.core.entity.Article;
import com.editorial.core.entity.ArticleStatus;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.Optional;
import java.util.UUID;

public interface ArticleRepository extends JpaRepository<Article, UUID> {

    Page<Article> findByStatusOrderByPublishedAtDesc(ArticleStatus status, Pageable pageable);

    Optional<Article> findByIdAndStatus(UUID id, ArticleStatus status);
}
