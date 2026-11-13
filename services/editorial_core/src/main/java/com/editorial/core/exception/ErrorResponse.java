package com.editorial.core.exception;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonInclude;
import lombok.Getter;
import org.springframework.http.HttpStatus;

@Getter
@JsonInclude(JsonInclude.Include.NON_NULL)
public class ErrorResponse {

    @JsonIgnore
    private final HttpStatus status;
    private final Integer code;
    private final String message;

    public ErrorResponse(HttpStatus status, Integer code, String message) {
        this.status = status;
        this.code = code;
        this.message = message;
    }

    public ErrorResponse(HttpStatus status, String message) {
        this(status, status.value(), message);
    }
}
