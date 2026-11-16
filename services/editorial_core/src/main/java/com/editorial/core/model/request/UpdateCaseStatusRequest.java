package com.editorial.core.model.request;

import com.editorial.core.entity.CaseStatus;
import jakarta.validation.constraints.NotNull;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class UpdateCaseStatusRequest {

    @NotNull
    private CaseStatus status;
}
