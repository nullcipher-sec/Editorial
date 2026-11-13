package com.editorial.core.model.response;

import org.springframework.data.domain.Page;

import java.util.List;

public record PageResponse<T>(
        List<T> content,
        int page,
        int size,
        long totalElements,
        int totalPages
) {
    public static <E, T> PageResponse<T> from(Page<E> page, List<T> mapped) {
        return new PageResponse<>(
                mapped,
                page.getNumber(),
                page.getSize(),
                page.getTotalElements(),
                page.getTotalPages()
        );
    }
}
