package com.editorial.core.exception;

import lombok.Getter;

@Getter
public class ControllerException extends Exception {

    private final ErrorResponse errorResponse;

    public ControllerException(ErrorResponse errorResponse) {
        super(errorResponse.getMessage(), null, false, false);
        this.errorResponse = errorResponse;
    }
}
