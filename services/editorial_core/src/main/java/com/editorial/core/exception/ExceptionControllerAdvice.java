package com.editorial.core.exception;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.ResponseEntity;
import org.springframework.http.converter.HttpMessageNotReadableException;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import org.springframework.web.method.annotation.MethodArgumentTypeMismatchException;
import org.springframework.web.servlet.resource.NoResourceFoundException;

@RestControllerAdvice
public class ExceptionControllerAdvice {

    private static final Logger logger = LoggerFactory.getLogger(ExceptionControllerAdvice.class);

    @ExceptionHandler(ControllerException.class)
    public ResponseEntity<ErrorResponse> handleControllerException(ControllerException ex) {
        ErrorResponse error = ex.getErrorResponse();
        if (error.getStatus().is5xxServerError()) {
            logger.error("ControllerException handled. status={}, code={}", error.getStatus().value(), error.getCode());
        }
        return ResponseEntity.status(error.getStatus()).body(error);
    }

    @ExceptionHandler(MethodArgumentNotValidException.class)
    public ResponseEntity<ErrorResponse> handleInvalidArgument(MethodArgumentNotValidException ex) {
        return ResponseEntity.status(ErrorResponses.BAD_REQUEST.getStatus()).body(ErrorResponses.BAD_REQUEST);
    }

    @ExceptionHandler(HttpMessageNotReadableException.class)
    public ResponseEntity<ErrorResponse> handleUnreadableBody(HttpMessageNotReadableException ex) {
        return ResponseEntity.status(ErrorResponses.BAD_REQUEST.getStatus()).body(ErrorResponses.BAD_REQUEST);
    }

    @ExceptionHandler(NoResourceFoundException.class)
    public ResponseEntity<ErrorResponse> handleNotFound(NoResourceFoundException ex) {
        return ResponseEntity.status(ErrorResponses.NOT_FOUND.getStatus()).body(ErrorResponses.NOT_FOUND);
    }

    @ExceptionHandler(MethodArgumentTypeMismatchException.class)
    public ResponseEntity<ErrorResponse> handleTypeMismatch(MethodArgumentTypeMismatchException ex) {
        return ResponseEntity.status(ErrorResponses.BAD_REQUEST.getStatus()).body(ErrorResponses.BAD_REQUEST);
    }

    @ExceptionHandler(Exception.class)
    public ResponseEntity<ErrorResponse> handleUnexpected(Exception ex) {
        logger.error("Unexpected error", ex);
        return ResponseEntity.status(ErrorResponses.INTERNAL_SERVER_ERROR.getStatus())
                .body(ErrorResponses.INTERNAL_SERVER_ERROR);
    }
}
